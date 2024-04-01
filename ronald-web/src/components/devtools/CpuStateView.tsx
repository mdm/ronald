import { hex } from "../../Utils";

type CpuState = {
  registerA: number;
  registerF: number;
  registerB: number;
  registerC: number;
  registerD: number;
  registerE: number;
  registerH: number;
  registerL: number;
  altRegisterA: number;
  altRegisterF: number;
  altRegisterB: number;
  altRegisterC: number;
  altRegisterD: number;
  altRegisterE: number;
  altRegisterH: number;
  altRegisterL: number;
  registerI: number;
  registerR: number;
  registerIx: number;
  registerIy: number;
  registerSp: number;
  registerPc: number;
  iff1: boolean;
  iff2: boolean;
  interruptMode: string;
  enableInterrupt: boolean;
  irqReceived: boolean;
};

type Props = {
  active: boolean;
  state?: CpuState;
};

const BASIC_REGISTERS = ["A", "B", "C", "D", "E", "H", "L"];
const SPECIAL_REGISTERS = ["IX", "IY", "SP", "PC"];
const FLAGS = ["S", "Z", "X", "H", "X", "P/V", "N", "C"];

const CpuStateView = (props: Props) => {
  return (
    <>
      <table class="border-separate">
        <thead>
          <tr>
            {BASIC_REGISTERS.map((register) => (
              <th class="bg-slate-300 w-10">{register}</th>
            ))}
            <th class="bg-slate-300 w-10">I</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerA, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerB, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerC, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerD, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerE, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerH, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerL, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerI, 2) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      <table class="border-separate">
        <thead>
          <tr>
            <th class="bg-slate-300" colspan="8">
              F
            </th>
          </tr>
          <tr>
            {FLAGS.map((flag) => (
              <th class="bg-slate-300 w-10">{flag}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x80) >> 7 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x40) >> 6 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x20) >> 5 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x10) >> 4 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x08) >> 3 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x04) >> 2 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x02) >> 1 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registerF & 0x01) >> 0 : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      <table class="border-separate">
        <thead>
          <tr>
            {BASIC_REGISTERS.map((register) => (
              <th class="bg-slate-300 w-10">{register}'</th>
            ))}
            <th class="bg-slate-300 w-10">R</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterA, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterB, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterC, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterD, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterE, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterH, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.altRegisterL, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerR, 2) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      <table class="border-separate">
        <thead>
          <tr>
            <th class="bg-slate-300" colspan="8">
              F'
            </th>
          </tr>
          <tr>
            {FLAGS.map((flag) => (
              <th class="bg-slate-300 w-10">{flag}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x80) >> 7 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x40) >> 6 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x20) >> 5 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x10) >> 4 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x08) >> 3 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x04) >> 2 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x02) >> 1 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.altRegisterF & 0x01) >> 0 : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      <table class="w-full border-separate">
        <thead>
          <tr>
            {SPECIAL_REGISTERS.map((register) => (
              <th class="bg-slate-300 w-20">{register}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerIx, 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerIy, 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerSp, 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registerPc, 4) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

export default CpuStateView;
