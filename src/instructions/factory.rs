use crate::instructions::Instruction;
use crate::instructions::constants::*;
use crate::instructions::load::*;
use crate::instructions::stores::*;
use crate::instructions::stack::*;
use crate::instructions::math::*;
use crate::instructions::conversions::*;
use crate::instructions::comparisons::*;
use crate::instructions::control::*;
use crate::instructions::extended::*;

const nop: NOP = NOP {};
const aconst_null: AconstNull = AconstNull {};
const iconst_m1: IconstM1 = IconstM1 {};
const iconst0: Iconst0 = Iconst0 {};
const iconst1: Iconst1 = Iconst1 {};
const iconst2: Iconst2 = Iconst2 {};
const iconst3: Iconst3 = Iconst3 {};
const iconst4: Iconst4 = Iconst4 {};
const iconst5: Iconst5 = Iconst5 {};
const lconst0: Lconst0 = Lconst0 {};
const lconst1: Lconst1 = Lconst1 {};
const fconst0: Fconst0 = Fconst0 {};
const fconst1: Fconst1 = Fconst1 {};
const fconst2: Fconst2 = Fconst2 {};
const dconst0: Dconst0 = Dconst0 {};
const dconst1: Dconst1 = Dconst1 {};
const BIPush: BIPush = BIPush { val: 0 };
const SIPush: SIPush = SIPush { val: 0 };
const iload: ILOAD = ILOAD { index: 0 };
const lload: LLOAD = LLOAD { index: 0 };
const fload: FLOAD = FLOAD { index: 0 };
const dload: DLOAD = DLOAD { index: 0 };
const aload: ALOAD = ALOAD { index: 0 };
const iload0: ILOAD0 = ILOAD0 {};
const iload1: ILOAD1 = ILOAD1 {};
const iload2: ILOAD2 = ILOAD2 {};
const iload3: ILOAD3 = ILOAD3 {};
const lload0: LLOAD0 = LLOAD0 {};
const lload1: LLOAD1 = LLOAD1 {};
const lload2: LLOAD2 = LLOAD2 {};
const lload3: LLOAD3 = LLOAD3 {};
const fload0: FLOAD0 = FLOAD0 {};
const fload1: FLOAD1 = FLOAD1 {};
const fload2: FLOAD2 = FLOAD2 {};
const fload3: FLOAD3 = FLOAD3 {};
const dload0: DLOAD0 = DLOAD0 {};
const dload1: DLOAD1 = DLOAD1 {};
const dload2: DLOAD2 = DLOAD2 {};
const dload3: DLOAD3 = DLOAD3 {};
const aload0: ALOAD0 = ALOAD0 {};
const aload1: ALOAD1 = ALOAD1 {};
const aload2: ALOAD2 = ALOAD2 {};
const aload3: ALOAD3 = ALOAD3 {};
const istore: ISTORE = ISTORE { index: 0 };
const lstore: LSTORE = LSTORE { index: 0 };
const fstore: FSTORE = FSTORE { index: 0 };
const dstore: DSTORE = DSTORE { index: 0 };
const astore: ASTORE = ASTORE { index: 0 };
const istore0: ISTORE0 = ISTORE0 {};
const istore1: ISTORE1 = ISTORE1 {};
const istore2: ISTORE2 = ISTORE2 {};
const istore3: ISTORE3 = ISTORE3 {};
const lstore0: LSTORE0 = LSTORE0 {};
const lstore1: LSTORE1 = LSTORE1 {};
const lstore2: LSTORE2 = LSTORE2 {};
const lstore3: LSTORE3 = LSTORE3 {};
const fstore0: FSTORE0 = FSTORE0 {};
const fstore1: FSTORE1 = FSTORE1 {};
const fstore2: FSTORE2 = FSTORE2 {};
const fstore3: FSTORE3 = FSTORE3 {};
const dstore0: DSTORE0 = DSTORE0 {};
const dstore1: DSTORE1 = DSTORE1 {};
const dstore2: DSTORE2 = DSTORE2 {};
const dstore3: DSTORE3 = DSTORE3 {};
const astore0: ASTORE0 = ASTORE0 {};
const astore1: ASTORE1 = ASTORE1 {};
const astore2: ASTORE2 = ASTORE2 {};
const astore3: ASTORE3 = ASTORE3 {};
const pop: POP = POP {};
const pop2: POP2 = POP2 {};
const dup: DUP = DUP {};
const dupX1: DupX1 = DupX1 {};
const dupX2: DupX2 = DupX2 {};
const dup2: DUP2 = DUP2 {};
const dup2X1: Dup2X1 = Dup2X1 {};
const dup2X2: Dup2X2 = Dup2X2 {};
const swap: SWAP = SWAP {};
const iadd: IADD = IADD {};
const ladd: LADD = LADD {};
const fadd: FADD = FADD {};
const dadd: DADD = DADD {};
const isub: ISUB = ISUB {};
const lsub: LSUB = LSUB {};
const fsub: FSUB = FSUB {};
const dsub: DSUB = DSUB {};
const imul: IMUL = IMUL {};
const lmul: LMUL = LMUL {};
const fmul: FMUL = FMUL {};
const dmul: DMUL = DMUL {};
const idiv: IDIV = IDIV {};
const ldiv: LDIV = LDIV {};
const fdiv: FDIV = FDIV {};
const ddiv: DDIV = DDIV {};
const irem: IREM = IREM {};
const lrem: LREM = LREM {};
const frem: FREM = FREM {};
const drem: DREM = DREM {};
const ineg: INEG = INEG {};
const lneg: LNEG = LNEG {};
const fneg: FNEG = FNEG {};
const dneg: DNEG = DNEG {};
const ishl: ISHL = ISHL {};
const lshl: LSHL = LSHL {};
const ishr: ISHR = ISHR {};
const lshr: LSHR = LSHR {};
const iushr: IUSHR = IUSHR {};
const lushr: LUSHR = LUSHR {};
const iand: IAND = IAND {};
const land: LAND = LAND {};
const ior: IOR = IOR {};
const lor: LOR = LOR {};
const ixor: IXOR = IXOR {};
const lxor: LXOR = LXOR {};
const iinc: IINC = IINC { idx: 0, cst: 0 };
const i2l: I2L = I2L {};
const i2f: I2F = I2F {};
const i2d: I2D = I2D {};
const l2i: L2I = L2I {};
const l2f: L2F = L2F {};
const l2d: L2D = L2D {};
const f2i: F2I = F2I {};
const f2l: I2F = I2F {};
const f2d: F2D = F2D {};
const d2i: D2I = D2I {};
const d2l: D2L = D2L {};
const d2f: D2F = D2F {};
const i2b: I2B = I2B {};
const i2c: I2C = I2C {};
const i2s: I2S = I2S {};
const lcmp: LCMP = LCMP {};
const fcmpl: FCMPL = FCMPL {};
const fcmpg: FCMPG = FCMPG {};
const dcmpl: DCMPL = DCMPL {};
const dcmpg: DCMPG = DCMPG {};
const ifeq: IFEQ = IFEQ { offset: 0 };
const ifne: IFNE = IFNE { offset: 0 };
const iflt: IFLT = IFLT { offset: 0 };
const ifge: IFGE = IFGE { offset: 0 };
const ifgt: IFGT = IFGT { offset: 0 };
const ifle: IFLE = IFLE { offset: 0 };
const ificmpeq: IF_ICMPEQ = IF_ICMPEQ { offset: 0 };
const ificmpne: IF_ICMPNE = IF_ICMPNE { offset: 0 };
const ificmplt: IF_ICMPLT = IF_ICMPLT { offset: 0 };
const ificmpge: IF_ICMPGE = IF_ICMPGE { offset: 0 };
const ificmpgt: IF_ICMPGT = IF_ICMPGT { offset: 0 };
const ificmple: IF_ICMPLE = IF_ICMPLE { offset: 0 };
const ifacmpeq: IF_ACMPEQ = IF_ACMPEQ { offset: 0 };
const ifacmpne: IF_ACMPNE = IF_ACMPNE { offset: 0 };
const goto: GOTO = GOTO { offset: 0 };
//const table_switch: TABLE_SWITCH = TABLE_SWITCH { default_offset: 0, low: 0, high: 0, jump_offsets: vec![] };
//const lookup_switch: LOOKUP_SWITCH = LOOKUP_SWITCH { default_offset: 0, npairs: 0, match_offsets: vec![] };
//const wide: WIDE = WIDE { modified_instruction: Box::new(()) };
const ifnull: IFNULL = IFNULL { offset: 0 };
const ifnonnull: IFNONNULL = IFNONNULL { offset: 0 };
const gotow: GOTOW = GOTOW { offset: 0 };

pub struct InstFactory {}

impl InstFactory {
    pub fn new(opcode: u8) -> Box<dyn Instruction> {
        return match opcode {
            0x00 => Box::new(nop),
            0x01 => Box::new(aconst_null),
            0x02 => Box::new(iconst_m1),
            0x03 => Box::new(iconst0),
            0x04 => Box::new(iconst1),
            0x05 => Box::new(iconst2),
            0x06 => Box::new(iconst3),
            0x07 => Box::new(iconst4),
            0x08 => Box::new(iconst5),
            0x09 => Box::new(lconst0),
            0x0a => Box::new(lconst1),
            0x0b => Box::new(fconst0),
            0x0c => Box::new(fconst1),
            0x0d => Box::new(fconst2),
            0x0e => Box::new(dconst0),
            0x0f => Box::new(dconst1),
            0x10 => Box::new(BIPush),
            0x11 => Box::new(SIPush),
            //0x12 => Box::new(dconst1),
            //0x13 => Box::new(BIPush),
            //0x14 => Box::new(SIPush),
            0x15 => Box::new(iload),
            0x16 => Box::new(lload),
            0x17 => Box::new(fload),
            0x18 => Box::new(dload),
            0x19 => Box::new(aload),
            0x1a => Box::new(iload0),
            0x1b => Box::new(iload1),
            0x1c => Box::new(iload2),
            0x1d => Box::new(iload3),
            0x1e => Box::new(lload0),
            0x1f => Box::new(lload1),
            0x20 => Box::new(lload2),
            0x21 => Box::new(lload3),
            0x22 => Box::new(fload0),
            0x23 => Box::new(fload1),
            0x24 => Box::new(fload2),
            0x25 => Box::new(fload3),
            0x26 => Box::new(dload0),
            0x27 => Box::new(dload1),
            0x28 => Box::new(dload2),
            0x29 => Box::new(dload3),
            0x2a => Box::new(aload0),
            0x2b => Box::new(aload1),
            0x2c => Box::new(aload2),
            0x2d => Box::new(aload3),
            //0x2e => Box::new(aload3),
            //0x2f => Box::new(aload3),
            //0x30 => Box::new(aload3),
            //0x31 => Box::new(aload3),
            //0x32 => Box::new(aload3),
            //0x33 => Box::new(aload3),
            //0x34 => Box::new(aload3),
            //0x35 => Box::new(aload3),
            0x36 => Box::new(istore),
            0x37 => Box::new(lstore),
            0x38 => Box::new(fstore),
            0x39 => Box::new(dstore),
            0x3a => Box::new(astore),
            0x3b => Box::new(istore0),
            0x3c => Box::new(istore1),
            0x3d => Box::new(istore2),
            0x33 => Box::new(istore3),
            0x3f => Box::new(lstore0),
            0x40 => Box::new(lstore1),
            0x41 => Box::new(lstore2),
            0x42 => Box::new(lstore3),
            0x43 => Box::new(fstore0),
            0x44 => Box::new(fstore1),
            0x45 => Box::new(fstore2),
            0x46 => Box::new(fstore3),
            0x47 => Box::new(dstore0),
            0x48 => Box::new(dstore1),
            0x49 => Box::new(dstore2),
            0x4a => Box::new(dstore3),
            0x4b => Box::new(astore0),
            0x4c => Box::new(astore1),
            0x4d => Box::new(astore2),
            0x4e => Box::new(astore3),
            //0x4f => Box::new(aload3),
            //0x50 => Box::new(aload3),
            //0x51 => Box::new(aload3),
            //0x52 => Box::new(aload3),
            //0x53 => Box::new(aload3),
            //0x54 => Box::new(aload3),
            //0x55 => Box::new(aload3),
            //0x56 => Box::new(aload3),
            0x57 => Box::new(pop),
            0x58 => Box::new(pop2),
            0x59 => Box::new(dup),
            0x5a => Box::new(dupX1),
            0x5b => Box::new(dupX2),
            0x5c => Box::new(dup2),
            0x5d => Box::new(dup2X1),
            0x5e => Box::new(dup2X2),
            0x5f => Box::new(swap),
            0x60 => Box::new(iadd),
            0x61 => Box::new(ladd),
            0x62 => Box::new(fadd),
            0x63 => Box::new(dadd),
            0x64 => Box::new(isub),
            0x65 => Box::new(lsub),
            0x66 => Box::new(fsub),
            0x67 => Box::new(dsub),
            0x68 => Box::new(imul),
            0x69 => Box::new(lmul),
            0x6a => Box::new(fmul),
            0x6b => Box::new(dmul),
            0x6c => Box::new(idiv),
            0x6d => Box::new(ldiv),
            0x6e => Box::new(fdiv),
            0x6f => Box::new(ddiv),
            0x70 => Box::new(irem),
            0x71 => Box::new(lrem),
            0x72 => Box::new(frem),
            0x73 => Box::new(drem),
            0x74 => Box::new(ineg),
            0x75 => Box::new(lneg),
            0x76 => Box::new(fneg),
            0x77 => Box::new(dneg),
            0x78 => Box::new(ishl),
            0x79 => Box::new(lshl),
            0x7a => Box::new(ishr),
            0x7b => Box::new(lshr),
            0x7c => Box::new(iushr),
            0x7d => Box::new(lushr),
            0x7e => Box::new(iand),
            0x7f => Box::new(land),
            0x80 => Box::new(ior),
            0x81 => Box::new(lor),
            0x82 => Box::new(ixor),
            0x83 => Box::new(lxor),
            0x84 => Box::new(iinc),
            0x85 => Box::new(i2l),
            0x86 => Box::new(i2f),
            0x87 => Box::new(i2d),
            0x88 => Box::new(l2i),
            0x89 => Box::new(l2f),
            0x8a => Box::new(l2d),
            0x8b => Box::new(f2i),
            0x8c => Box::new(f2l),
            0x8d => Box::new(f2d),
            0x8e => Box::new(d2i),
            0x8f => Box::new(d2l),
            0x90 => Box::new(d2f),
            0x91 => Box::new(i2b),
            0x92 => Box::new(i2c),
            0x93 => Box::new(i2s),
            0x94 => Box::new(lcmp),
            0x95 => Box::new(fcmpl),
            0x96 => Box::new(fcmpg),
            0x97 => Box::new(dcmpl),
            0x98 => Box::new(dcmpg),
            0x99 => Box::new(ifeq),
            0x9a => Box::new(ifne),
            0x9b => Box::new(iflt),
            0x9c => Box::new(ifge),
            0x9d => Box::new(ifgt),
            0x9e => Box::new(ifle),
            0x9f=> Box::new(ificmpeq),
            0xa0=> Box::new(ificmpne),
            0xa1=> Box::new(ificmplt),
            0xa2=> Box::new(ificmpge),
            0xa3=> Box::new(ificmpgt),
            0xa4=> Box::new(ificmple),
            0xa5=> Box::new(ifacmpeq),
            0xa6=> Box::new(ifacmpne),
            0xa7=> Box::new(goto),
            //0xa8=> Box::new(goto),
            //0xa9=> Box::new(goto),
            0xaa=> Box::new(TABLE_SWITCH { default_offset: 0, low: 0, high: 0, jump_offsets: vec![] }),
            0xab=> Box::new(LOOKUP_SWITCH { default_offset: 0, npairs: 0, match_offsets: vec![] }),
            //0xac=> Box::new(goto),
            //0xad=> Box::new(goto),
            //0xae=> Box::new(goto),
            //0xaf=> Box::new(goto),
            //0xb0=> Box::new(goto),
            //0xb1=> Box::new(goto),
            //0xb2=> Box::new(GET_STATIC),
            //0xb3=> Box::new(goto),
            //0xb4=> Box::new(goto),
            //0xb5=> Box::new(goto),
            //0xb6=> Box::new(goto),
            //0xb7=> Box::new(goto),
            //0xb8=> Box::new(goto),
            //0xb9=> Box::new(goto),
            //0xba=> Box::new(goto),
            //0xbb=> Box::new(goto),
            //0xbc=> Box::new(goto),
            //0xbd=> Box::new(goto),
            //0xbe=> Box::new(goto),
            //0xbf=> Box::new(goto),
            //0xc0=> Box::new(goto),
            //0xc1=> Box::new(goto),
            //0xc2=> Box::new(goto),
            //0xc3=> Box::new(goto),
            0xc4=> {
                Box::new(WIDE { modified_instruction: Box::new(nop) })
            },
            //0xc5=> Box::new(goto),
            0xc6=> Box::new(ifnull),
            0xc7=> Box::new(ifnonnull),
            0xc8=> Box::new(gotow),
            //0xc9=> Box::new(goto),
            //0xca=> Box::new(goto),
            //0xfe=> Box::new(goto),
            //0xff=> Box::new(goto),
            _ => panic!("Unsupported opcode: {}", opcode)
        };
    }
}