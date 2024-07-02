import { hex } from "../../Utils";

type CpuState = {
  // decoder: DecoderState;
  registers: number[];
  halted: boolean;
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
      Registers
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
              {props.active ? hex(props.state!.registers[0] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[1] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[1] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[2] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[2] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[3] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[3] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[8] & 0xff, 2) : "-"}
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
              {props.active ? (props.state!.registers[0] & 0x80) >> 7 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x40) >> 6 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x20) >> 5 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x10) >> 4 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x08) >> 3 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x04) >> 2 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x02) >> 1 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[0] & 0x01) >> 0 : "-"}
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
              {props.active ? hex(props.state!.registers[4] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[5] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[5] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[6] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[6] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[7] >> 8, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[7] & 0xff, 2) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[9] & 0xff, 2) : "-"}
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
              {props.active ? (props.state!.registers[4] & 0x80) >> 7 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x40) >> 6 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x20) >> 5 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x10) >> 4 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x08) >> 3 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x04) >> 2 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x02) >> 1 : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.registers[4] & 0x01) >> 0 : "-"}
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
              {props.active ? hex(props.state!.registers[10], 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[11], 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[12], 4) : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? hex(props.state!.registers[13], 4) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      Interrupts
      <table class="w-full border-separate">
        <thead>
          <tr>
            <th class="bg-slate-300 w-10">Mode</th>
            <th class="bg-slate-300 w-10">IFF1</th>
            <th class="bg-slate-300 w-10">IFF2</th>
            <th class="bg-slate-300 w-10">IRQ</th>
            <th class="bg-slate-300 w-10">Enable</th>
            <th class="bg-slate-300 w-10">Halted</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td class="bg-slate-100 text-center">
              {props.active ? props.state!.interruptMode : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.iff1 ? "true" : "false") : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.iff2 ? "true" : "false") : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active
                ? props.state!.irqReceived
                  ? "true"
                  : "false"
                : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active
                ? props.state!.enableInterrupt
                  ? "true"
                  : "false"
                : "-"}
            </td>
            <td class="bg-slate-100 text-center">
              {props.active ? (props.state!.halted ? "true" : "false") : "-"}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

export default CpuStateView;
