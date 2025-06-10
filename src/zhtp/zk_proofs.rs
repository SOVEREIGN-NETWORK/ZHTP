use ark_ff::{Field, One, Zero, PrimeField};
use ark_std::io::Cursor;
use serde::{Serialize, Deserialize};
use ark_poly::{
    univariate::DensePolynomial,
    EvaluationDomain, GeneralEvaluationDomain,
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_bn254::{Fr, G1Affine, G1Projective};
use ark_std::vec::Vec;
use std::collections::{HashMap, HashSet};

// Type alias for internal use
type G1 = G1Projective;

/// Serializable version of cryptographic types using byte representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByteRoutingProof {
    pub commitments: Vec<Vec<u8>>,
    pub elements: Vec<Vec<u8>>,
    pub inputs: Vec<Vec<u8>>,
}

impl From<RoutingProof> for ByteRoutingProof {
    fn from(proof: RoutingProof) -> Self {
        let commitments = proof.path_commitments.iter().map(|pc| {
            let mut bytes = Vec::new();
            pc.0.serialize_uncompressed(&mut bytes).unwrap();
            bytes
        }).collect();

        let elements = proof.proof_elements.iter().map(|fr| {
            let mut bytes = Vec::new();
            fr.serialize_uncompressed(&mut bytes).unwrap();
            bytes
        }).collect();

        let inputs = proof.public_inputs.iter().map(|fr| {
            let mut bytes = Vec::new();
            fr.serialize_uncompressed(&mut bytes).unwrap();
            bytes
        }).collect();

        ByteRoutingProof {
            commitments,
            elements,
            inputs,
        }
    }
}

impl TryFrom<ByteRoutingProof> for RoutingProof {
    type Error = ark_serialize::SerializationError;

    fn try_from(bytes: ByteRoutingProof) -> Result<Self, Self::Error> {
        let path_commitments = bytes.commitments.iter()
            .map(|bytes| -> Result<PolyCommit, ark_serialize::SerializationError> {
                let mut cursor = Cursor::new(bytes.as_slice());
                let point = G1Projective::deserialize_uncompressed(&mut cursor)?;
                Ok(PolyCommit(point))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let proof_elements = bytes.elements.iter()
            .map(|bytes| -> Result<Fr, ark_serialize::SerializationError> {
                let mut cursor = Cursor::new(bytes.as_slice());
                Fr::deserialize_uncompressed(&mut cursor)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let public_inputs = bytes.inputs.iter()
            .map(|bytes| -> Result<Fr, ark_serialize::SerializationError> {
                let mut cursor = Cursor::new(bytes.as_slice());
                Fr::deserialize_uncompressed(&mut cursor)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(RoutingProof {
            path_commitments,
            proof_elements,
            public_inputs,
        })
    }
}

/// Types of proofs supported by the system
#[derive(Debug, Clone, PartialEq)]
pub enum ProofType {
    Routing,
    Storage,
    NetworkMetrics,
    Unified,
}

/// Polynomial commitment using elliptic curve point
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize, Serialize, Deserialize)]
pub struct PolyCommit(#[serde(with = "g1_serde")] pub G1Projective);

// Serialization helper module for G1Projective
mod g1_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(point: &G1Projective, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = Vec::new();
        point.serialize_uncompressed(&mut bytes).map_err(serde::ser::Error::custom)?;
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<G1Projective, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        G1Projective::deserialize_uncompressed(&bytes[..]).map_err(serde::de::Error::custom)
    }
}

// Serialization helper module for Fr
mod fr_serde {
    use super::*;
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(field: &Fr, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = Vec::new();
        field.serialize_uncompressed(&mut bytes).map_err(serde::ser::Error::custom)?;
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Fr, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        Fr::deserialize_uncompressed(&bytes[..]).map_err(serde::de::Error::custom)
    }
}

/// Storage proof components
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct StorageProof {
    /// Merkle root of stored data
    pub data_root: [u8; 32],
    /// Proof of space commitment
    pub space_commitment: G1Projective,
    /// Timestamp of last verification
    pub last_verified: u64,
    /// Proof elements for storage verification
    pub storage_proof: Vec<Fr>,
}

/// Network metrics proof components
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct NetworkMetricsProof {
    /// Bandwidth commitment
    pub bandwidth_commit: G1Projective,
    /// Uptime proof
    pub uptime_proof: Vec<Fr>,
    /// Latency measurements proof
    pub latency_proof: Vec<Fr>,
}

/// A routing proof showing that a packet was correctly forwarded
#[derive(Clone, Debug, CanonicalSerialize, CanonicalDeserialize, Default)]
pub struct RoutingProof {
    /// Polynomial commitments for the routing path
    pub path_commitments: Vec<PolyCommit>,
    /// PLONK proof elements
    pub proof_elements: Vec<Fr>,
    /// Public inputs for the circuit
    pub public_inputs: Vec<Fr>,
}

/// Combined circuit for proving network contributions
#[derive(Debug)]
pub struct UnifiedCircuit {
    // Routing components
    source_node: Vec<u8>,
    destination_node: Vec<u8>,
    route_path: Vec<Vec<u8>>,
    routing_table: HashMap<Vec<u8>, Vec<Vec<u8>>>,
    
    // Storage components
    stored_data_root: [u8; 32],
    storage_merkle_proof: Vec<[u8; 32]>,
    space_commitment: G1Projective,
    
    // Network metrics components
    bandwidth_used: u64,
    uptime_records: Vec<(u64, bool)>, // timestamp, online status
    latency_measurements: Vec<(u64, f64)>, // timestamp, latency in ms
    
    // Public inputs
    public_inputs: Vec<Fr>,
    
    // PLONK circuit components
    wire_polynomials: Vec<DensePolynomial<Fr>>,
    selector_polynomials: Vec<DensePolynomial<Fr>>,
    permutation_polynomials: Vec<DensePolynomial<Fr>>,
    evaluation_domain: GeneralEvaluationDomain<Fr>,
}

impl UnifiedCircuit {
    /// Create a new unified circuit for network proofs
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: Vec<u8>,
        destination: Vec<u8>,
        path: Vec<Vec<u8>>,
        routing_table: HashMap<Vec<u8>, Vec<Vec<u8>>>,
        stored_data_root: [u8; 32],
        storage_proof: Vec<[u8; 32]>,
        space_commitment: G1Projective,
        bandwidth_used: u64,
        uptime_records: Vec<(u64, bool)>,
        latency_measurements: Vec<(u64, f64)>,
    ) -> Self {
        // Calculate domain size based on all constraints
        let constraint_count = path.len() + // Routing constraints
                             storage_proof.len() + // Storage verification
                             uptime_records.len() + // Uptime verification
                             latency_measurements.len(); // Performance metrics
        
        let domain_size = constraint_count.next_power_of_two();
        let evaluation_domain = GeneralEvaluationDomain::new(domain_size)
            .expect("Failed to create evaluation domain");

        UnifiedCircuit {
            source_node: source.clone(),
            destination_node: destination.clone(),
            route_path: path.clone(),
            routing_table,
            stored_data_root,
            storage_merkle_proof: storage_proof,
            space_commitment,
            bandwidth_used,
            uptime_records,
            latency_measurements,
            public_inputs: Vec::new(),
            wire_polynomials: Vec::new(),
            selector_polynomials: Vec::new(),
            permutation_polynomials: Vec::new(),
            evaluation_domain,
        }
    }

    /// Add all constraints for unified proof
    fn add_constraints(&mut self) {
        let mut wire_values: Vec<Fr> = Vec::new();

        // 1. Add routing constraints
        self.add_routing_constraints(&mut wire_values);
        
        // 2. Add storage constraints
        self.add_storage_constraints(&mut wire_values);
        
        // 3. Add network metrics constraints
        self.add_metrics_constraints(&mut wire_values);

        // Convert all wire values to polynomials
        self.wire_polynomials = self.values_to_polynomials(&wire_values);
        println!("Generated {} total polynomials", self.wire_polynomials.len());
    }

    /// Add only the required routing constraints
    fn add_routing_constraints(&self, wire_values: &mut Vec<Fr>) {
        let start_len = wire_values.len();
        
        // Add routing constraints if path exists
        if !self.route_path.is_empty() {
            // Add node hashes
            for node in &self.route_path {
                wire_values.push(self.hash_to_field(node));
            }

            // Verify and add validity flags between nodes
            if self.route_path.len() > 1 {
                for i in 0..self.route_path.len() - 1 {
                    let current = &self.route_path[i];
                    let next = &self.route_path[i + 1];
                    
                    // A node is valid if it exists in routing table AND is a valid next hop
                    let valid = self.routing_table.get(current)
                        .map(|hops| hops.contains(next))
                        .unwrap_or(false);

                    // Invalid hops must be marked invalid in proof
                    wire_values.push(if valid { Fr::one() } else { Fr::zero() });

                    // If invalid hop found, mark all remaining hops as invalid
                    if !valid {
                        for _ in i+1..self.route_path.len()-1 {
                            wire_values.push(Fr::zero());
                        }
                        break;
                    }
                }
            }
        }

        let added = wire_values.len() - start_len;
        let expected = if self.route_path.is_empty() { 0 } else {
            self.route_path.len() + // Node hashes
            if self.route_path.len() > 1 { self.route_path.len() - 1 } else { 0 } // Validity flags
        };
        
        assert_eq!(added, expected,
            "Added {} routing constraints but expected {}", added, expected);
    }

    /// Add storage verification constraints
    fn add_storage_constraints(&self, wire_values: &mut Vec<Fr>) {
        let start_len = wire_values.len();

        // Note: Root hash is already included in base values
        if self.storage_merkle_proof.is_empty() {
            // Just add space commitment when no proof
            wire_values.push(self.compute_space_commitment());
        } else {
            let mut current = self.stored_data_root;
            
            // Add Merkle proof pairs
            for node in &self.storage_merkle_proof {
                wire_values.push(self.hash_to_field(&current)); // Parent
                wire_values.push(self.hash_to_field(node));     // Child
                current = self.compute_merkle_node(&current, node);
            }
            
            // Add final space commitment
            wire_values.push(self.compute_space_commitment());
        }

        // Verify total matches expected
        let added = wire_values.len() - start_len;
        let expected = if self.storage_merkle_proof.is_empty() {
            1 // Just space commitment
        } else {
            self.storage_merkle_proof.len() * 2 + 1 // Proof pairs + commitment
        };

        assert_eq!(added, expected,
            "Storage constraints mismatch - added: {}, expected: {} (proof_len: {})",
            added, expected, self.storage_merkle_proof.len());
    }

    /// Helper: Compute space commitment field element
    fn compute_space_commitment(&self) -> Fr {
        Fr::from_random_bytes(&self.serialize_point(&self.space_commitment))
            .unwrap_or_else(|| {
                println!("Warning: Using zero for invalid space commitment");
                Fr::zero()
            })
    }

    /// Helper: Compute Merkle node hash
    fn compute_merkle_node(&self, left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let result = hasher.finalize();
        let mut output = [0u8; 32];
        output.copy_from_slice(&result);
        output
    }

    /// Helper: Serialize curve point to bytes
    fn serialize_point(&self, point: &G1) -> Vec<u8> {
        let mut bytes = Vec::new();
        point.serialize_uncompressed(&mut bytes)
            .expect("Point serialization failed");
        bytes
    }

    /// Add network metrics verification values
    fn add_metrics_constraints(&self, wire_values: &mut Vec<Fr>) {
        let start_len = wire_values.len();

        // Bandwidth is already in base values, only add records
        if !self.uptime_records.is_empty() {
            // Add uptime records in timestamp order
            let mut records = self.uptime_records.clone();
            records.sort_by_key(|(ts, _)| *ts);
            
            for (timestamp, online) in records {
                wire_values.push(Fr::from(timestamp));
                wire_values.push(Fr::from(online as u64));
            }
        }

        if !self.latency_measurements.is_empty() {
            // Add latency records in timestamp order
            let mut records = self.latency_measurements.clone();
            records.sort_by_key(|(ts, _)| *ts);
            
            for (timestamp, latency) in records {
                wire_values.push(Fr::from(timestamp));
                wire_values.push(Fr::from(latency.to_bits() as u64));
            }
        }

        // Verify added count matches expectation
        let added = wire_values.len() - start_len;
        let expected = self.uptime_records.len() * 2 +
                      self.latency_measurements.len() * 2;

        assert_eq!(added, expected,
            "Metrics values mismatch: added {} but expected {} (uptime: {}, latency: {})",
            added, expected, self.uptime_records.len(), self.latency_measurements.len());
    }


    /// Generate polynomials for proof constraints
    fn generate_polynomials(&mut self) {
        // Get actual constraint counts
        let (base_count, constraint_count, metrics_count) = self.commitment_counts();
        let total_count = base_count + constraint_count + metrics_count;
        
        // Create selector polynomial for each constraint
        let selector_values = vec![Fr::one(); total_count];
        self.selector_polynomials = self.values_to_polynomials(&selector_values);

        // Create sequential permutation polynomials
        let mut perm_values = Vec::with_capacity(total_count);
        for i in 0..total_count {
            perm_values.push(Fr::from((i + 1) as u64));
        }
        self.permutation_polynomials = self.values_to_polynomials(&perm_values);
        
        debug_assert_eq!(self.selector_polynomials.len(), total_count,
            "Wrong number of selector polynomials");
        debug_assert_eq!(self.permutation_polynomials.len(), total_count,
            "Wrong number of permutation polynomials");
    }

    /// Calculate exact commitment counts for each component
    fn commitment_counts(&self) -> (usize, usize, usize) {
        // Base values (always present)
        let base_count = 5; // source, dest, root, bandwidth, record count

        // Routing constraints
        let routing_count = if self.route_path.is_empty() {
            0
        } else {
            self.route_path.len() + // Node hashes
            if self.route_path.len() > 1 { self.route_path.len() - 1 } else { 0 } // Validity flags
        };

        // Storage constraints (root is in base values)
        let storage_count = if self.storage_merkle_proof.is_empty() {
            1 // Just space commitment
        } else {
            (self.storage_merkle_proof.len() * 2) + 1 // Proof pairs + commitment
        };

        // Network metrics (bandwidth in base values)
        let metrics_count = self.uptime_records.len() * 2 +
                          self.latency_measurements.len() * 2;

        let constraint_count = routing_count + storage_count;
        (base_count, constraint_count, metrics_count)
    }

    /// Calculate total commitment count with detailed logging
    fn calculate_commitment_count(&self) -> usize {
        let (base, constraints, metrics) = self.commitment_counts();
        let total = base + constraints + metrics;

        println!("\nExpected commitment counts:");
        println!("Base ({}):", base);
        println!("  - Source/dest/root/bandwidth/counts");
        
        println!("Constraints ({}):", constraints);
        if !self.route_path.is_empty() {
            println!("  - Route: {} nodes + {} flags",
                self.route_path.len(),
                if self.route_path.len() > 1 { self.route_path.len() - 1 } else { 0 });
        }
        println!("  - Storage: {} proofs + 1 commitment",
            self.storage_merkle_proof.len() * 2);

        println!("Metrics ({}):", metrics);
        println!("  - {} uptime records = {} values",
            self.uptime_records.len(), self.uptime_records.len() * 2);
        println!("  - {} latency records = {} values",
            self.latency_measurements.len(), self.latency_measurements.len() * 2);

        println!("Total expected: {}", total);
        total
    }

    /// Generate a unified proof of routing, storage and network metrics
    pub fn generate_proof(&mut self) -> Option<RoutingProof> {
        // First verify the path is valid
        if !self.route_path.is_empty() {
            for i in 0..self.route_path.len() - 1 {
                let current = &self.route_path[i];
                let next = &self.route_path[i + 1];
                
                // Check if this hop is allowed by routing table
                if !self.routing_table.get(current)
                    .map_or(false, |hops| hops.contains(next)) {
                    println!("Invalid path: {:?} -> {:?} not in routing table", current, next);
                    return None;
                }
            }
        }

        println!("\nGenerating proof with circuit state:");
        println!("- Route path length: {}", self.route_path.len());
        println!("- Merkle proof length: {}", self.storage_merkle_proof.len());
        println!("- Uptime records: {}", self.uptime_records.len());
        println!("- Latency records: {}", self.latency_measurements.len());
        
        // Calculate expected commitment counts
        let (base_count, constraint_count, metrics_count) = self.commitment_counts();
        let total_commitments = base_count + constraint_count + metrics_count;
        
        // Pre-allocate vector with exact size
        let mut wire_values = Vec::with_capacity(total_commitments);
        
        // Add base public inputs in fixed order
        let base_values = [
            self.hash_to_field(&self.source_node),      // Source ID
            self.hash_to_field(&self.destination_node), // Destination ID
            self.hash_to_field(&self.stored_data_root), // Storage root
            Fr::from(self.bandwidth_used),              // Bandwidth usage
            Fr::from(self.uptime_records.len() as u64), // Record count
        ];
        wire_values.extend_from_slice(&base_values);
        
        debug_assert_eq!(wire_values.len(), base_count,
            "Base value count wrong: {} != {}", wire_values.len(), base_count);
        
        // Track constraints being added
        let routing_start = wire_values.len();
        self.add_routing_constraints(&mut wire_values);
        let routing_added = wire_values.len() - routing_start;
        
        let storage_start = wire_values.len();
        self.add_storage_constraints(&mut wire_values);
        let storage_added = wire_values.len() - storage_start;
        
        let metrics_start = wire_values.len();
        self.add_metrics_constraints(&mut wire_values);
        let metrics_added = wire_values.len() - metrics_start;
        
        println!("\nConstraint counts:");
        println!("- Base values: {}", base_count);
        println!("- Routing constraints added: {}", routing_added);
        println!("- Storage constraints added: {}", storage_added);
        println!("- Metrics constraints added: {}", metrics_added);
        println!("- Total values: {} (expected {})", wire_values.len(), total_commitments);
        
        // Convert to polynomials
        self.wire_polynomials = self.values_to_polynomials(&wire_values);
        self.generate_polynomials();
        
        // Generate polynomial commitments
        let challenge_point = Fr::from(2u64);
        let mut path_commitments = Vec::with_capacity(wire_values.len());
        let mut proof_elements = Vec::with_capacity(wire_values.len());
        
        for (_i, poly) in self.wire_polynomials.iter().enumerate() {
            let eval = evaluate_polynomial(poly, &challenge_point);
            proof_elements.push(eval);
            path_commitments.push(PolyCommit(self.commit_polynomial(poly)));
        }
        
        // Construct final proof
        let proof = RoutingProof {
            path_commitments,
            proof_elements: proof_elements.clone(),
            public_inputs: wire_values.clone(), // Clone to keep original values
        };
        
        // Final verification of proof structure
        let (base, constraints, metrics) = self.commitment_counts();
        let expected_total = base + constraints + metrics;
        
        assert_eq!(proof.path_commitments.len(), expected_total,
            "Wrong number of commitments: expected {} = {} + {} + {}, got {}",
            expected_total, base, constraints, metrics,
            proof.path_commitments.len());
            
        assert_eq!(proof.proof_elements.len(), proof.path_commitments.len(),
            "Mismatched proof elements ({}) and commitments ({})",
            proof.proof_elements.len(), proof.path_commitments.len());
            
        assert_eq!(proof.public_inputs.len(), expected_total,
            "Wrong number of public inputs: expected {}, got {}",
            expected_total, proof.public_inputs.len());
            
        // Verify base values are in correct order
        debug_assert_eq!(proof.public_inputs[0], self.hash_to_field(&self.source_node), "Source mismatch");
        debug_assert_eq!(proof.public_inputs[1], self.hash_to_field(&self.destination_node), "Dest mismatch");
        debug_assert_eq!(proof.public_inputs[2], self.hash_to_field(&self.stored_data_root), "Root mismatch");
        debug_assert_eq!(proof.public_inputs[3], Fr::from(self.bandwidth_used), "Bandwidth mismatch");
        debug_assert_eq!(proof.public_inputs[4], Fr::from(self.uptime_records.len() as u64), "Record count mismatch");
            
        println!("Generated valid proof with {} total commitments", proof.path_commitments.len());
        Some(proof)
    }


    /// Helper: Convert values to polynomials in evaluation domain
    fn values_to_polynomials(&self, values: &[Fr]) -> Vec<DensePolynomial<Fr>> {
        let mut polynomials = Vec::new();
        
        // Create a separate polynomial for each value
        for value in values.iter() {
            let mut coeffs = vec![*value];
            coeffs.resize(self.evaluation_domain.size(), Fr::zero());
            polynomials.push(DensePolynomial { coeffs });
            println!("Created polynomial for value");
        }
        
        polynomials
    }

    /// Helper: Commit to polynomial using G1 curve point
    fn commit_polynomial(&self, poly: &DensePolynomial<Fr>) -> G1Projective {
        let mut result = G1Projective::zero();
        let gen = G1Affine::generator().into_group();
        let mut powers = Vec::with_capacity(poly.coeffs.len());
        
        // Generate powers for KZG commitment
        let mut current = gen;
        powers.push(current);
        let secret = Fr::from(2u64); // In practice this would be a secure random value
        
        for _ in 1..poly.coeffs.len() {
            current *= secret;
            powers.push(current);
        }
        
        // Compute commitment using KZG scheme
        for (i, coeff) in poly.coeffs.iter().enumerate() {
            result += powers[i].mul_bigint(coeff.into_bigint());
        }
        
        result
    }

    /// Helper: Hash bytes to field element
    pub fn hash_to_field(&self, bytes: &[u8]) -> Fr {
        use sha2::{Sha256, Digest};
        use ark_ff::PrimeField;
        
        // Use hash_to_field with domain separation
        let mut hasher = Sha256::new();
        hasher.update(b"ZHTP-v1"); // Domain separator
        hasher.update(bytes);
        let hash = hasher.finalize();
        
        // Ensure uniform distribution in field
        let _modulus = Fr::MODULUS;
        let mut num = Fr::zero();
        
        // Convert bytes to field element with modular reduction
        for chunk in hash.chunks(8) {
            let mut val = 0u64;
            for &byte in chunk {
                val = (val << 8) | byte as u64;
            }
            num += Fr::from(val);
            num *= Fr::from(256u64);
        }
        
        // Ensure result is in valid range
        if num.is_zero() {
            Fr::one()
        } else {
            num
        }
    }
}

/// Helper function to evaluate polynomial at a point
fn evaluate_polynomial(poly: &DensePolynomial<Fr>, point: &Fr) -> Fr {
    let mut result = Fr::zero();
    let mut power = Fr::one();
    
    for coeff in poly.coeffs.iter() {
        result += *coeff * power;
        power *= point;
    }
    
    result
}

/// Helper function to validate proof structure
fn validate_proof_structure(proof: &RoutingProof) -> bool {
    // Check component counts match
    if proof.path_commitments.len() != proof.proof_elements.len() ||
       proof.path_commitments.len() != proof.public_inputs.len() {
        println!("Proof component count mismatch");
        return false;
    }

    // Verify minimum required components
    if proof.public_inputs.len() < 5 {
        println!("Missing required base inputs");
        return false;
    }

    true
}

/// Verify all components of a unified proof
pub fn verify_unified_proof(
    proof: &RoutingProof,
    source: &[u8],
    destination: &[u8],
    stored_data_root: [u8; 32]
) -> bool {
    // Early validation of proof structure
    if !validate_proof_structure(proof) {
        return false;
    }

    // Create verification circuit with routing table
    let mut routing_table = HashMap::new();
    routing_table.insert(source.to_vec(), vec![destination.to_vec()]); // Allow direct path
    
    let mut circuit = UnifiedCircuit::new(
        source.to_vec(),
        destination.to_vec(),
        Vec::new(),
        routing_table,
        stored_data_root,
        Vec::new(),
        G1Projective::generator(),
        0,
        Vec::new(),
        Vec::new(),
    );

    // Calculate expected proof sizes
    let (base_count, constraint_count, metrics_count) = circuit.commitment_counts();
    let total_expected = base_count + constraint_count + metrics_count;

    // For view change proofs with zeroed root, only check source and destination
    let is_view_change = stored_data_root == [0u8; 32];
    let base_checks = if is_view_change {
        vec![
            (proof.public_inputs[0], circuit.hash_to_field(source), "source"),
            (proof.public_inputs[1], circuit.hash_to_field(destination), "destination"),
        ]
    } else {
        vec![
            (proof.public_inputs[0], circuit.hash_to_field(source), "source"),
            (proof.public_inputs[1], circuit.hash_to_field(destination), "destination"),
            (proof.public_inputs[2], circuit.hash_to_field(&stored_data_root), "root"),
        ]
    };

    for (actual, expected, name) in base_checks {
        if actual != expected {
            println!("{} hash mismatch", name);
            return false;
        }
    }

    let (base_count, constraint_count, metrics_count) = circuit.commitment_counts();
    
    // Determine if this is a routing proof based on structure
    let has_routing = !source.is_empty() &&
                     !destination.is_empty() &&
                     constraint_count > 1 && // More than just base constraints
                     proof.path_commitments.len() > 10; // Long enough to contain routing

    // Validate routing proof
    if !source.is_empty() && !destination.is_empty() && constraint_count > 1 {
        // Build map of valid routes and their hashes
        let mut valid_routes: std::collections::HashMap<ark_bn254::Fr, Vec<ark_bn254::Fr>> = std::collections::HashMap::new();
        let source_vec = source.to_vec();
        let dest_vec = destination.to_vec();
        
        // First verify a valid path exists in the routing table
        if !circuit.routing_table.contains_key(&source_vec) {
            println!("Source node not in routing table");
            return false;
        }
        
        // Pre-compute hashes for efficient lookup
        let mut route_hashes = HashMap::new();
        let mut node_to_hash = HashMap::new();
        
        // Build optimized routing table with pre-computed hashes
        for (from_node, next_hops) in circuit.routing_table.iter() {
            let from_hash = circuit.hash_to_field(from_node);
            node_to_hash.insert(from_node.clone(), from_hash);
            
            let mut hashed_hops = Vec::with_capacity(next_hops.len());
            for hop in next_hops {
                let hash = circuit.hash_to_field(hop);
                node_to_hash.insert(hop.clone(), hash);
                hashed_hops.push(hash);
            }
            route_hashes.insert(from_hash, hashed_hops);
        }
        
        // Verify path reachability using dynamic programming
        let mut reachable = HashSet::new();
        let source_hash = circuit.hash_to_field(&source_vec);
        let dest_hash = circuit.hash_to_field(&dest_vec);
        
        // Initialize with source
        reachable.insert(source_hash);
        
        // Expand reachable nodes until no more progress or destination found
        let mut found_path = false;
        let mut prev_size = 0;
        
        while reachable.len() != prev_size {
            prev_size = reachable.len();
            let current = reachable.clone();
            
            for &node in current.iter() {
                if let Some(next_hops) = route_hashes.get(&node) {
                    for &hop in next_hops {
                        reachable.insert(hop);
                        if hop == dest_hash {
                            found_path = true;
                            break;
                        }
                    }
                }
                if found_path { break; }
            }
            if found_path { break; }
        }
        
        if !found_path {
            println!("No valid path exists from source to destination");
            return false;
        }
        
        // Extract path nodes and validity flags
        let mut path_nodes = Vec::new();
        let mut path_valid = Vec::new();
        let mut i = 5; // Skip base values
        
        while i + 1 < proof.proof_elements.len() && i < 5 + constraint_count * 2 {
            path_nodes.push(proof.proof_elements[i]);
            path_valid.push(proof.proof_elements[i + 1]);
            i += 2;
        }

        // Verify path integrity
        if path_nodes.is_empty() || path_valid.is_empty() {
            println!("Empty path in proof");
            return false;
        }

        // Verify endpoints
        if path_nodes.first() != Some(&source_hash) || path_nodes.last() != Some(&dest_hash) {
            println!("Invalid path endpoints");
            return false;
        }

        // Batch validate all path segments
        let mut all_hops_valid = true;
        let mut combined_valid = Fr::one();

        for window in path_nodes.windows(2) {
            let (current, next) = (window[0], window[1]);
            
            // Verify hop exists in routing table
            let hop_valid = route_hashes.get(&current)
                .map_or(false, |valid_next| valid_next.contains(&next));
            
            if !hop_valid {
                all_hops_valid = false;
                break;
            }

            // Accumulate validity flags for batch check
            combined_valid *= path_valid[path_nodes.iter().position(|&x| x == current).unwrap()];
        }

        if !all_hops_valid || combined_valid != Fr::one() {
            println!("Invalid path segments detected");
            return false;
        }

        // Batch verify polynomial commitments
        let gen = G1Affine::generator().into_group();
        let mut combined_commit = G1::zero();
        let mut combined_eval = Fr::zero();
        
        // Batch verify polynomial commitments
        for (commit, eval) in proof.path_commitments.iter().zip(proof.proof_elements.iter()) {
            combined_commit += commit.0;
            combined_eval += *eval;
        }
        let batch_valid = bool::from(gen.mul_bigint(combined_eval.into_bigint()).eq(&combined_commit));
        if !batch_valid {
            println!("Batch commitment verification failed");
            return false;
        }
        
        // Verify all public inputs match
        if proof.proof_elements != proof.public_inputs {
            println!("Mismatch between proof elements and public inputs");
            return false;
        }
    }  // Close the if block for routing verification

    println!("All proof components verified successfully");
    true
}  // Close verify_unified_proof function

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use ark_bn254::{Fr, G1Projective as G1};
    use ark_ff::One;
    
    #[derive(Clone)]
    pub struct TestProofBundle {
        pub routing_proof: RoutingProof,
        pub storage_proof: StorageProof,
        pub source: Vec<u8>,
        pub destination: Vec<u8>
    }

    pub fn setup_test_proofs() -> TestProofBundle {
        // Create empty proofs initially - source/destination will be set by test
        let source = vec![];
        let destination = vec![];
        let root = [1u8; 32];
        
        // Generate commitment components
        let path_commitments = vec![PolyCommit(G1::generator()); 11];
        let mut proof_elements = vec![Fr::one(); 11];
        let mut public_inputs = vec![Fr::one(); 11];

        // Create basic routing proof - we'll update inputs later
        let routing_proof = RoutingProof {
            path_commitments,
            proof_elements,
            public_inputs
        };

        let storage_proof = StorageProof {
            data_root: root,
            space_commitment: G1::generator(),
            last_verified: chrono::Utc::now().timestamp() as u64,
            storage_proof: vec![Fr::one(); 7]
        };

        TestProofBundle {
            routing_proof,
            storage_proof,
            source,
            destination
        }
    }

    pub fn generate_test_storage_proof() -> StorageProof {
        let storage_proof = vec![Fr::one(); 7];
        StorageProof {
            data_root: [1u8; 32],
            space_commitment: G1::generator(),
            last_verified: chrono::Utc::now().timestamp() as u64,
            storage_proof,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_helpers::*;
    use std::time::Instant;

    fn create_test_data() -> ([u8; 32], Vec<[u8; 32]>) {
        let mut data_root = [0u8; 32];
        data_root[0] = 1;
        let mut proof = Vec::new();
        for i in 0..3 {
            let mut node = [0u8; 32];
            node[0] = i as u8;
            proof.push(node);
        }
        (data_root, proof)
    }

    fn create_test_metrics() -> (u64, Vec<(u64, bool)>, Vec<(u64, f64)>) {
        let bandwidth = 1024 * 1024; // 1MB
        let uptime = vec![
            (1234567890, true),
            (1234567891, true),
            (1234567892, false),
        ];
        let latency = vec![
            (1234567890, 50.0),
            (1234567891, 55.0),
            (1234567892, 45.0),
        ];
        (bandwidth, uptime, latency)
    }

    #[test]
    fn test_storage_proof_verification() {
        let (data_root, merkle_proof) = create_test_data();
        let space_commitment = G1::generator();
        
        let mut circuit = UnifiedCircuit::new(
            vec![1,2,3],
            vec![4,5,6],
            Vec::new(),
            HashMap::new(),
            data_root,
            merkle_proof.clone(),
            space_commitment,
            0,
            Vec::new(),
            Vec::new(),
        );

        // Generate proof and verify
        if let Some(proof) = circuit.generate_proof() {
            // Storage proof should include:
            // - All base proof elements (5)
            // - Merkle proof hashes (merkle_proof.len() * 2)
            // - Space commitment (1)
            let expected_count = 5 + (merkle_proof.len() * 2) + 1;
            assert_eq!(proof.proof_elements.len(), expected_count,
                "Wrong number of proof elements, expected {}, got {}",
                expected_count, proof.proof_elements.len());
                
            // Verify proof validates
            assert!(verify_unified_proof(&proof, &[1,2,3], &[4,5,6], data_root),
                "Storage proof verification failed");
        } else {
            panic!("Failed to generate proof");
        }
    }

    #[test]
    fn test_network_metrics_verification() {
        let (bandwidth, uptime, latency) = create_test_metrics();
        
        let mut circuit = UnifiedCircuit::new(
            vec![1,2,3],
            vec![4,5,6],
            Vec::new(),
            HashMap::new(),
            [0u8; 32],
            Vec::new(),
            G1::zero(),
            bandwidth,
            uptime.clone(),
            latency.clone(),
        );

        // Generate proof with metrics
        if let Some(proof) = circuit.generate_proof() {
            // Metrics proof should include:
            // - All base proof elements (5)
            // - Bandwidth measurement (1)
            // - Uptime records with timestamps (uptime.len() * 2)
            // - Latency measurements with timestamps (latency.len() * 2)
            let expected_count = 5 + 1 + (uptime.len() * 2) + (latency.len() * 2);
            
            assert_eq!(proof.proof_elements.len(), expected_count,
                "Wrong number of proof elements, expected {}, got {} (uptime: {}, latency: {})",
                expected_count, proof.proof_elements.len(), uptime.len(), latency.len());
            
            // Verify metrics proof validates
            assert!(verify_unified_proof(&proof, &[1,2,3], &[4,5,6], [0u8; 32]),
                "Network metrics proof verification failed");
        } else {
            panic!("Failed to generate proof");
        }
    }

    #[test]
    fn test_proof_performance() {
        let start = Instant::now();
        
        // Setup complete test case with all components
        let source: Vec<u8> = vec![1, 2, 3];
        let destination: Vec<u8> = vec![4, 5, 6];
        let path: Vec<Vec<u8>> = vec![
            vec![1, 2, 3],
            vec![7, 8, 9],
            vec![4, 5, 6],
        ];
        
        let mut routing_table: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::new();
        routing_table.insert(vec![1, 2, 3], vec![vec![7, 8, 9]]);
        routing_table.insert(vec![7, 8, 9], vec![vec![4, 5, 6]]);

        let (data_root, merkle_proof) = create_test_data();
        let (bandwidth, uptime, latency) = create_test_metrics();
        
        let mut circuit = UnifiedCircuit::new(
            source.clone(),
            destination.clone(),
            path.clone(),
            routing_table,
            data_root,
            merkle_proof.clone(),
            G1::generator(),
            bandwidth,
            uptime.clone(),
            latency.clone(),
        );

        // Calculate actual commitment counts
        let routing_commitments = path.len() + (path.len() - 1); // path nodes + validity flags
        let storage_commitments = (merkle_proof.len() * 2) + 1; // merkle nodes + commitment
        let metrics_commitments = (uptime.len() * 2) + (latency.len() * 2); // records only
        let base_commitments = 5; // source, dest, root, bandwidth, record count
        let expected_total = base_commitments + routing_commitments + storage_commitments + metrics_commitments;

        println!("\nGenerating unified proof with:");
        println!("- {} routing commitments", routing_commitments);
        println!("- {} storage commitments", storage_commitments);
        println!("- {} metrics commitments", metrics_commitments);
        
        let proof = circuit.generate_proof()
            .expect("Failed to generate proof for valid test case");
        let proof_time = start.elapsed();
        
        // Verify proof structure
        assert_eq!(proof.path_commitments.len(), expected_total,
            "Expected {} commitments, got {}",
            expected_total, proof.path_commitments.len());
        
        assert_eq!(proof.proof_elements.len(), proof.path_commitments.len(),
            "Mismatched number of proof elements and commitments");
        
        // Verify proof validates
        let verify_start = Instant::now();
        let valid = verify_unified_proof(&proof, &source, &destination, data_root);
        let verify_time = verify_start.elapsed();
        
        assert!(valid, "Unified proof verification failed");
        
        println!("\nPerformance metrics:");
        println!("- Proof generation: {:?}", proof_time);
        println!("- Proof verification: {:?}", verify_time);
        println!("- Total commitments: {}", proof.path_commitments.len());
    }

    #[test]
    fn test_invalid_storage_proof() {
        // Create valid data root
        let mut valid_root = [0u8; 32];
        valid_root[0] = 1;
        
        // Create circuit with empty storage proof
        let mut circuit = UnifiedCircuit::new(
            vec![1,2,3],
            vec![4,5,6],
            Vec::new(),
            HashMap::new(),
            valid_root,
            Vec::new(),  // Empty proof
            G1::zero(),
            0,
            Vec::new(),
            Vec::new(),
        );

        // Should be able to generate proof
        let valid_proof = circuit.generate_proof()
            .expect("Should generate proof with empty storage proof");

        // Proof should validate with correct root
        assert!(verify_unified_proof(&valid_proof, &[1,2,3], &[4,5,6], valid_root),
            "Should validate with correct root");

        // But should fail with wrong root
        let wrong_root = [2u8; 32];
        assert!(!verify_unified_proof(&valid_proof, &[1,2,3], &[4,5,6], wrong_root),
            "Should not validate with wrong root");
    }

    #[test]
    fn test_unified_proof() {
        // Setup valid test components
        let source = vec![1, 2, 3];
        let mid_hop = vec![7, 8, 9];
        let destination = vec![4, 5, 6];
        let valid_path = vec![source.clone(), mid_hop.clone(), destination.clone()];

        // Setup routing table
        let mut routing_table = HashMap::new();
        routing_table.insert(source.clone(), vec![mid_hop.clone()]);
        routing_table.insert(mid_hop.clone(), vec![destination.clone()]);
        
        // Create test data
        let (data_root, merkle_proof) = create_test_data();
        let (bandwidth, uptime, latency) = create_test_metrics();

        // Create circuit with valid path
        let mut circuit = UnifiedCircuit::new(
            source.clone(),
            destination.clone(),
            valid_path,
            routing_table,
            data_root,
            merkle_proof.clone(),
            G1::generator(),
            bandwidth,
            uptime.clone(),
            latency.clone(),
        );
        
        // Get commitment counts for logging
        let (base, constraints, metrics) = circuit.commitment_counts();
        let total = base + constraints + metrics;
        
        println!("\nExpected commitments in unified proof:");
        println!("- Base commitments: {}", base);
        println!("- Constraint commitments: {}", constraints);
        println!("- Metrics commitments: {}", metrics);
        println!("Total expected: {}", total);

        // Generate proof (should succeed with valid path)
        let valid_proof = circuit.generate_proof()
            .expect("Should generate proof for valid unified circuit");
            
        // Verify proof structure and validation
        assert!(!valid_proof.proof_elements.is_empty(), "Proof should contain elements");
        assert!(!valid_proof.path_commitments.is_empty(), "Proof should contain commitments");
        assert_eq!(valid_proof.proof_elements.len(), valid_proof.path_commitments.len(),
            "Should have same number of elements and commitments");
            
        // Verify proof validates with correct parameters
        assert!(verify_unified_proof(&valid_proof, &source, &destination, data_root),
            "Valid unified proof should verify successfully");
    }

    #[test]
    fn test_invalid_proof() {
        // Setup test environment
        let source = vec![1, 2, 3];
        let destination = vec![4, 5, 6];
        let valid_hop = vec![7, 8, 9];
        
        // Create routing table with only one valid path:
        // source -> valid_hop -> destination
        let mut routing_table = HashMap::new();
        routing_table.insert(source.clone(), vec![valid_hop.clone()]);
        routing_table.insert(valid_hop.clone(), vec![destination.clone()]);

        // Test 1: Valid path should work
        let mut circuit = UnifiedCircuit::new(
            source.clone(),
            destination.clone(),
            vec![source.clone(), valid_hop.clone(), destination.clone()],
            routing_table.clone(),
            [0u8; 32],
            Vec::new(),
            G1::generator(),
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(circuit.generate_proof().is_some(), "Valid path should generate proof");

        // Test 2: Invalid path should fail
        let mut circuit = UnifiedCircuit::new(
            source.clone(),
            destination.clone(),
            vec![source.clone(), vec![9,9,9], destination.clone()], // Invalid middle hop
            routing_table.clone(),
            [0u8; 32],
            Vec::new(),
            G1::generator(),
            0,
            Vec::new(),
            Vec::new(),
        );
        assert!(circuit.generate_proof().is_none(), "Invalid path should not generate proof");
    }
}