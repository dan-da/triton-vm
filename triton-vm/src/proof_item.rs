use arbitrary::Arbitrary;
use strum::Display;
use strum::EnumCount;
use twenty_first::shared_math::b_field_element::BFieldElement;
use twenty_first::shared_math::bfield_codec::BFieldCodec;
use twenty_first::shared_math::tip5::Digest;
use twenty_first::shared_math::x_field_element::XFieldElement;

use crate::error::ProofStreamError;
use crate::error::ProofStreamError::UnexpectedItem;
use crate::fri::AuthenticationStructure;
use crate::stark::NUM_QUOTIENT_SEGMENTS;

type Result<T> = std::result::Result<T, ProofStreamError>;

/// A `FriResponse` is an `AuthenticationStructure` together with the values of the
/// revealed leaves of the Merkle tree. Together, they correspond to the
/// queried indices of the FRI codeword (of that round).
#[derive(Debug, Clone, PartialEq, Eq, Hash, BFieldCodec, Arbitrary)]
pub struct FriResponse {
    /// The authentication structure of the Merkle tree.
    pub auth_structure: AuthenticationStructure,
    /// The values of the opened leaves of the Merkle tree.
    pub revealed_leaves: Vec<XFieldElement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, EnumCount, BFieldCodec, Arbitrary)]
pub enum ProofItem {
    AuthenticationStructure(AuthenticationStructure),
    MasterBaseTableRows(Vec<Vec<BFieldElement>>),
    MasterExtTableRows(Vec<Vec<XFieldElement>>),
    OutOfDomainBaseRow(Vec<XFieldElement>),
    OutOfDomainExtRow(Vec<XFieldElement>),
    OutOfDomainQuotientSegments([XFieldElement; NUM_QUOTIENT_SEGMENTS]),
    MerkleRoot(Digest),
    Log2PaddedHeight(u32),
    QuotientSegmentsElements(Vec<[XFieldElement; NUM_QUOTIENT_SEGMENTS]>),
    FriCodeword(Vec<XFieldElement>),
    FriResponse(FriResponse),
}

impl ProofItem {
    /// Whether a given proof item should be considered in the Fiat-Shamir heuristic.
    /// The Fiat-Shamir heuristic is sound only if all elements in the (current) transcript are
    /// considered. However, certain elements indirectly appear more than once. For example, a
    /// Merkle root is a commitment to any number of elements. If the Merkle root is part of the
    /// transcript, has been considered in the Fiat-Shamir heuristic, and assuming collision
    /// resistance of the hash function in use, none of the committed-to elements have to be
    /// considered in the Fiat-Shamir heuristic again.
    /// This also extends to the authentication structure of these elements, et cetera.
    pub const fn include_in_fiat_shamir_heuristic(&self) -> bool {
        use ProofItem::*;
        match self {
            MerkleRoot(_) => true,
            OutOfDomainBaseRow(_) => true,
            OutOfDomainExtRow(_) => true,
            OutOfDomainQuotientSegments(_) => true,
            // all of the following are implied by a corresponding Merkle root
            AuthenticationStructure(_) => false,
            MasterBaseTableRows(_) => false,
            MasterExtTableRows(_) => false,
            Log2PaddedHeight(_) => false,
            QuotientSegmentsElements(_) => false,
            FriCodeword(_) => false,
            FriResponse(_) => false,
        }
    }

    pub fn as_authentication_structure(&self) -> Result<AuthenticationStructure> {
        match self {
            Self::AuthenticationStructure(caps) => Ok(caps.to_owned()),
            other => Err(UnexpectedItem("authentication structure", other.to_owned())),
        }
    }

    pub fn as_master_base_table_rows(&self) -> Result<Vec<Vec<BFieldElement>>> {
        match self {
            Self::MasterBaseTableRows(bss) => Ok(bss.to_owned()),
            other => Err(UnexpectedItem("master base table rows", other.to_owned())),
        }
    }

    pub fn as_master_ext_table_rows(&self) -> Result<Vec<Vec<XFieldElement>>> {
        match self {
            Self::MasterExtTableRows(xss) => Ok(xss.to_owned()),
            o => Err(UnexpectedItem("master extension table rows", o.to_owned())),
        }
    }

    pub fn as_out_of_domain_base_row(&self) -> Result<Vec<XFieldElement>> {
        match self {
            Self::OutOfDomainBaseRow(xs) => Ok(xs.to_owned()),
            other => Err(UnexpectedItem("out of domain base row", other.to_owned())),
        }
    }

    pub fn as_out_of_domain_ext_row(&self) -> Result<Vec<XFieldElement>> {
        match self {
            Self::OutOfDomainExtRow(xs) => Ok(xs.to_owned()),
            o => Err(UnexpectedItem("out of domain extension row", o.to_owned())),
        }
    }

    pub fn as_out_of_domain_quotient_segments(
        &self,
    ) -> Result<[XFieldElement; NUM_QUOTIENT_SEGMENTS]> {
        match self {
            Self::OutOfDomainQuotientSegments(xs) => Ok(*xs),
            other => Err(UnexpectedItem(
                "out of domain quotient segments",
                other.to_owned(),
            )),
        }
    }

    pub fn as_merkle_root(&self) -> Result<Digest> {
        match self {
            Self::MerkleRoot(bs) => Ok(*bs),
            other => Err(UnexpectedItem("merkle root", other.to_owned())),
        }
    }

    pub fn as_log2_padded_height(&self) -> Result<u32> {
        match self {
            Self::Log2PaddedHeight(log2_padded_height) => Ok(*log2_padded_height),
            other => Err(UnexpectedItem("log2 padded height", other.to_owned())),
        }
    }

    pub fn as_quotient_segments_elements(
        &self,
    ) -> Result<Vec<[XFieldElement; NUM_QUOTIENT_SEGMENTS]>> {
        match self {
            Self::QuotientSegmentsElements(xs) => Ok(xs.to_owned()),
            o => Err(UnexpectedItem("quotient segments' elements", o.to_owned())),
        }
    }

    pub fn as_fri_codeword(&self) -> Result<Vec<XFieldElement>> {
        match self {
            Self::FriCodeword(xs) => Ok(xs.to_owned()),
            other => Err(UnexpectedItem("FRI codeword", other.to_owned())),
        }
    }

    pub fn as_fri_response(&self) -> Result<FriResponse> {
        match self {
            Self::FriResponse(fri_proof) => Ok(fri_proof.to_owned()),
            other => Err(UnexpectedItem("FRI proof", other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::assert;
    use assert2::let_assert;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;
    use twenty_first::shared_math::tip5::Tip5;
    use twenty_first::shared_math::x_field_element::XFieldElement;
    use twenty_first::util_types::merkle_tree::CpuParallel;
    use twenty_first::util_types::merkle_tree::MerkleTree;
    use twenty_first::util_types::merkle_tree_maker::MerkleTreeMaker;

    use crate::proof::Proof;
    use crate::proof_stream::ProofStream;

    use super::*;

    #[derive(Debug, Clone, test_strategy::Arbitrary)]
    struct LeavedMerkleTreeTestData {
        #[strategy(1..=10_usize)]
        _tree_height: usize,

        #[strategy(vec(arb(), 1 << #_tree_height))]
        _leaves: Vec<XFieldElement>,

        #[strategy(vec(0..#_leaves.len(), 1..=#_leaves.len()))]
        _revealed_indices: Vec<usize>,

        #[strategy(Just(#_leaves.iter().map(|&x| x.into()).collect()))]
        _leaves_as_digests: Vec<Digest>,

        #[strategy(Just(CpuParallel::from_digests(&#_leaves_as_digests).unwrap()))]
        _merkle_tree: MerkleTree<Tip5>,

        #[strategy(Just(#_revealed_indices.iter().map(|&i| #_leaves[i]).collect()))]
        revealed_leaves: Vec<XFieldElement>,

        #[strategy(Just(#_merkle_tree.authentication_structure(&#_revealed_indices).unwrap()))]
        auth_structure: AuthenticationStructure,
    }

    #[proptest]
    fn serialize_fri_response_in_isolation(leaved_merkle_tree: LeavedMerkleTreeTestData) {
        let fri_response = FriResponse {
            auth_structure: leaved_merkle_tree.auth_structure,
            revealed_leaves: leaved_merkle_tree.revealed_leaves,
        };

        let encoding = fri_response.encode();
        let_assert!(Ok(decoding) = FriResponse::decode(&encoding));
        assert!(fri_response == *decoding);
    }

    #[proptest]
    fn serialize_fri_response_in_proof_stream(leaved_merkle_tree: LeavedMerkleTreeTestData) {
        let fri_response = FriResponse {
            auth_structure: leaved_merkle_tree.auth_structure,
            revealed_leaves: leaved_merkle_tree.revealed_leaves,
        };

        let mut proof_stream = ProofStream::<Tip5>::new();
        proof_stream.enqueue(ProofItem::FriResponse(fri_response.clone()));
        let proof: Proof = proof_stream.into();

        let_assert!(Ok(mut proof_stream) = ProofStream::<Tip5>::try_from(&proof));
        let_assert!(Ok(proof_item) = proof_stream.dequeue());
        let_assert!(Ok(fri_response_) = proof_item.as_fri_response());
        assert!(fri_response == fri_response_);
    }

    #[proptest]
    fn serialize_authentication_structure_in_isolation(
        leaved_merkle_tree: LeavedMerkleTreeTestData,
    ) {
        let auth_structure = leaved_merkle_tree.auth_structure;
        let encoding = auth_structure.encode();
        let_assert!(Ok(decoding) = AuthenticationStructure::decode(&encoding));
        assert!(auth_structure == *decoding);
    }

    #[proptest]
    fn serialize_authentication_structure_in_proof_stream(
        leaved_merkle_tree: LeavedMerkleTreeTestData,
    ) {
        let auth_structure = leaved_merkle_tree.auth_structure;
        let mut proof_stream = ProofStream::<Tip5>::new();
        proof_stream.enqueue(ProofItem::AuthenticationStructure(auth_structure.clone()));
        let proof: Proof = proof_stream.into();

        let_assert!(Ok(mut proof_stream) = ProofStream::<Tip5>::try_from(&proof));
        let_assert!(Ok(proof_item) = proof_stream.dequeue());
        let_assert!(Ok(auth_structure_) = proof_item.as_authentication_structure());
        assert!(auth_structure == auth_structure_);
    }

    #[test]
    fn interpreting_a_merkle_root_as_anything_else_gives_appropriate_error() {
        let fake_root = Digest::default();
        let proof_item = ProofItem::MerkleRoot(fake_root);
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_authentication_structure());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_fri_response());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_master_base_table_rows());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_master_ext_table_rows());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_out_of_domain_base_row());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_out_of_domain_ext_row());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_out_of_domain_quotient_segments());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_log2_padded_height());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_quotient_segments_elements());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_fri_codeword());
        assert!(let Err(UnexpectedItem(_, _)) = proof_item.as_fri_response());
    }
}
