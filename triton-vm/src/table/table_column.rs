//! Enums that convert table column names into `usize` indices. Allows addressing columns by name
//! rather than their hard-to-remember index.

use std::hash::Hash;

use strum_macros::Display;
use strum_macros::EnumCount as EnumCountMacro;
use strum_macros::EnumIter;

use crate::table::master_table::EXT_HASH_TABLE_START;
use crate::table::master_table::EXT_JUMP_STACK_TABLE_START;
use crate::table::master_table::EXT_OP_STACK_TABLE_START;
use crate::table::master_table::EXT_PROCESSOR_TABLE_START;
use crate::table::master_table::EXT_PROGRAM_TABLE_START;
use crate::table::master_table::EXT_RAM_TABLE_START;
use crate::table::master_table::EXT_U32_TABLE_START;
use crate::table::master_table::HASH_TABLE_START;
use crate::table::master_table::JUMP_STACK_TABLE_START;
use crate::table::master_table::OP_STACK_TABLE_START;
use crate::table::master_table::PROCESSOR_TABLE_START;
use crate::table::master_table::PROGRAM_TABLE_START;
use crate::table::master_table::RAM_TABLE_START;
use crate::table::master_table::U32_TABLE_START;

// -------- Program Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum ProgramBaseTableColumn {
    Address,
    Instruction,
    LookupMultiplicity,
    IsPadding,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum ProgramExtTableColumn {
    InstructionLookupServerLogDerivative,
}

// -------- Processor Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum ProcessorBaseTableColumn {
    CLK,
    IsPadding,
    PreviousInstruction,
    IP,
    CI,
    NIA,
    IB0,
    IB1,
    IB2,
    IB3,
    IB4,
    IB5,
    IB6,
    IB7,
    JSP,
    JSO,
    JSD,
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
    ST8,
    ST9,
    ST10,
    ST11,
    ST12,
    ST13,
    ST14,
    ST15,
    OSP,
    OSV,
    HV0,
    HV1,
    HV2,
    HV3,
    RAMP,
    RAMV,
    /// The number of clock jump differences of magnitude `CLK` in all memory-like tables.
    ClockJumpDifferenceLookupMultiplicity,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum ProcessorExtTableColumn {
    InputTableEvalArg,
    OutputTableEvalArg,
    InstructionLookupClientLogDerivative,
    OpStackTablePermArg,
    RamTablePermArg,
    JumpStackTablePermArg,

    /// For copying the hash function's input to the hash coprocessor.
    HashInputEvalArg,
    /// For copying the hash digest from the hash coprocessor.
    HashDigestEvalArg,
    /// For copying the RATE next to-be-absorbed to the hash coprocessor and the RATE squeezed
    /// elements from the hash coprocessor, depending on the executed instruction.
    SpongeEvalArg,

    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the U32 Table.
    U32LookupClientLogDerivative,

    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the memory-like tables.
    ClockJumpDifferenceLookupServerLogDerivative,
}

// -------- OpStack Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum OpStackBaseTableColumn {
    CLK,
    IB1ShrinkStack,
    OSP,
    OSV,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum OpStackExtTableColumn {
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

// -------- RAM Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum RamBaseTableColumn {
    CLK,
    PreviousInstruction,
    RAMP,
    RAMV,
    InverseOfRampDifference,
    BezoutCoefficientPolynomialCoefficient0,
    BezoutCoefficientPolynomialCoefficient1,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum RamExtTableColumn {
    RunningProductOfRAMP,
    FormalDerivative,
    BezoutCoefficient0,
    BezoutCoefficient1,
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

// -------- JumpStack Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum JumpStackBaseTableColumn {
    CLK,
    CI,
    JSP,
    JSO,
    JSD,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum JumpStackExtTableColumn {
    RunningProductPermArg,
    /// The (running sum of the) logarithmic derivative for the clock jump difference Lookup
    /// Argument with the Processor Table.
    ClockJumpDifferenceLookupClientLogDerivative,
}

// -------- Hash Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum HashBaseTableColumn {
    ROUNDNUMBER,
    CI,
    STATE0,
    STATE1,
    STATE2,
    STATE3,
    STATE4,
    STATE5,
    STATE6,
    STATE7,
    STATE8,
    STATE9,
    STATE10,
    STATE11,
    STATE12,
    STATE13,
    STATE14,
    STATE15,
    CONSTANT0A,
    CONSTANT1A,
    CONSTANT2A,
    CONSTANT3A,
    CONSTANT4A,
    CONSTANT5A,
    CONSTANT6A,
    CONSTANT7A,
    CONSTANT8A,
    CONSTANT9A,
    CONSTANT10A,
    CONSTANT11A,
    CONSTANT12A,
    CONSTANT13A,
    CONSTANT14A,
    CONSTANT15A,
    CONSTANT0B,
    CONSTANT1B,
    CONSTANT2B,
    CONSTANT3B,
    CONSTANT4B,
    CONSTANT5B,
    CONSTANT6B,
    CONSTANT7B,
    CONSTANT8B,
    CONSTANT9B,
    CONSTANT10B,
    CONSTANT11B,
    CONSTANT12B,
    CONSTANT13B,
    CONSTANT14B,
    CONSTANT15B,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum HashExtTableColumn {
    HashInputRunningEvaluation,
    HashDigestRunningEvaluation,

    SpongeRunningEvaluation,
}

// -------- U32 Table --------

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum U32BaseTableColumn {
    /// Marks the beginning of an independent section within the U32 table.
    CopyFlag,

    /// The number of bits that LHS and RHS have already been shifted by.
    Bits,

    /// The inverse-or-zero of the difference between
    /// 1. the first disallowed number of bits to shift LHS and RHS by, _i.e.,_ 33, and
    /// 2. the number of bits that LHS and RHS have already been shifted by.
    BitsMinus33Inv,

    /// Current Instruction, the instruction the processor is currently executing.
    CI,

    /// Left-hand side of the operation.
    LHS,

    /// The inverse-or-zero of LHS. Needed to check whether `LHS` is unequal to 0.
    LhsInv,

    /// Right-hand side of the operation.
    RHS,

    /// The inverse-or-zero of RHS. Needed to check whether `RHS` is unequal to 0.
    RhsInv,

    /// The result (or intermediate result) of the instruction requested by the processor.
    Result,

    /// The number of times the processor has executed the current instruction with the same
    /// arguments.
    LookupMultiplicity,
}

#[repr(usize)]
#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCountMacro, Hash)]
pub enum U32ExtTableColumn {
    /// The (running sum of the) logarithmic derivative for the Lookup Argument with the
    /// Processor Table.
    LookupServerLogDerivative,
}

// --------------------------------------------------------------------

/// A trait for the columns of the master base table. This trait is implemented for all enums
/// relating to the base tables. This trait provides two methods:
/// - one to get the index of the column in the ”local“ base table, _i.e., not the master base
/// table, and
/// - one to get the index of the column in the master base table.
pub trait MasterBaseTableColumn {
    /// The index of the column in the ”local“ base table, _i.e., not the master base table.
    fn base_table_index(&self) -> usize;

    /// The index of the column in the master base table.
    fn master_base_table_index(&self) -> usize;
}

impl MasterBaseTableColumn for ProgramBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        PROGRAM_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for ProcessorBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        PROCESSOR_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for OpStackBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        OP_STACK_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for RamBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        RAM_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for JumpStackBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        JUMP_STACK_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for HashBaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        HASH_TABLE_START + self.base_table_index()
    }
}

impl MasterBaseTableColumn for U32BaseTableColumn {
    #[inline]
    fn base_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_base_table_index(&self) -> usize {
        U32_TABLE_START + self.base_table_index()
    }
}

// --------------------------------------------------------------------

/// A trait for the columns in the master extension table. This trait is implemented for all enums
/// relating to the extension tables. The trait provides two methods:
/// - one to get the index of the column in the “local” extension table, _i.e._, not the master
/// extension table, and
/// - one to get the index of the column in the master extension table.
pub trait MasterExtTableColumn {
    /// The index of the column in the “local” extension table, _i.e._, not the master extension
    /// table.
    fn ext_table_index(&self) -> usize;

    /// The index of the column in the master extension table.
    fn master_ext_table_index(&self) -> usize;
}

impl MasterExtTableColumn for ProgramExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_PROGRAM_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for ProcessorExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_PROCESSOR_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for OpStackExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_OP_STACK_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for RamExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_RAM_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for JumpStackExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_JUMP_STACK_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for HashExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_HASH_TABLE_START + self.ext_table_index()
    }
}

impl MasterExtTableColumn for U32ExtTableColumn {
    #[inline]
    fn ext_table_index(&self) -> usize {
        (*self) as usize
    }

    #[inline]
    fn master_ext_table_index(&self) -> usize {
        EXT_U32_TABLE_START + self.ext_table_index()
    }
}

// --------------------------------------------------------------------

#[cfg(test)]
mod table_column_tests {
    use strum::IntoEnumIterator;

    use crate::table::hash_table;
    use crate::table::jump_stack_table;
    use crate::table::op_stack_table;
    use crate::table::processor_table;
    use crate::table::program_table;
    use crate::table::ram_table;

    use super::*;

    #[test]
    fn column_max_bound_matches_table_width() {
        assert_eq!(
            program_table::BASE_WIDTH,
            ProgramBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "ProgramTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            processor_table::BASE_WIDTH,
            ProcessorBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "ProcessorTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            op_stack_table::BASE_WIDTH,
            OpStackBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "OpStackTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            ram_table::BASE_WIDTH,
            RamBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "RamTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            jump_stack_table::BASE_WIDTH,
            JumpStackBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "JumpStackTable's BASE_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            hash_table::BASE_WIDTH,
            HashBaseTableColumn::iter()
                .last()
                .unwrap()
                .base_table_index()
                + 1,
            "HashTable's BASE_WIDTH is 1 + its max column index",
        );

        assert_eq!(
            program_table::EXT_WIDTH,
            ProgramExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "ProgramTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            processor_table::EXT_WIDTH,
            ProcessorExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "ProcessorTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            op_stack_table::EXT_WIDTH,
            OpStackExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "OpStack:Table's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            ram_table::EXT_WIDTH,
            RamExtTableColumn::iter().last().unwrap().ext_table_index() + 1,
            "RamTable's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            jump_stack_table::EXT_WIDTH,
            JumpStackExtTableColumn::iter()
                .last()
                .unwrap()
                .ext_table_index()
                + 1,
            "JumpStack:Table's EXT_WIDTH is 1 + its max column index",
        );
        assert_eq!(
            hash_table::EXT_WIDTH,
            HashExtTableColumn::iter().last().unwrap().ext_table_index() + 1,
            "HashTable's EXT_WIDTH is 1 + its max column index",
        );
    }

    #[test]
    fn master_base_table_is_contiguous() {
        let mut expected_column_index = 0;
        for column in ProgramBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in ProcessorBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in OpStackBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in RamBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in JumpStackBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
        for column in HashBaseTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_base_table_index());
            expected_column_index += 1;
        }
    }

    #[test]
    fn master_ext_table_is_contiguous() {
        let mut expected_column_index = 0;
        for column in ProgramExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in ProcessorExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in OpStackExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in RamExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in JumpStackExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
        for column in HashExtTableColumn::iter() {
            assert_eq!(expected_column_index, column.master_ext_table_index());
            expected_column_index += 1;
        }
    }
}
