import { Show } from "solid-js";
import { hex } from "../../Utils";

type Instruction = {
  address: number;
  instruction: string;
};

type Props = {
  active: boolean;
  disassembly?: Instruction[];
};

const DisassemblyView = (props: Props) => {
  return (
    <Show when={props.active}>
      <table class="w-full">
        <thead>
          <tr>
            <th class="bg-slate-300 text-left">Address</th>
            <th class="bg-slate-300 text-left">Instruction</th>
            <th class="bg-slate-300 text-left">Operands</th>
          </tr>
        </thead>
        <tbody class="font-mono">
          {props.disassembly?.map((instruction) => (
            <tr class="bg-slate-100 first:bg-blue-400">
              <td class="pr-4">0x{hex(instruction.address, 4)}</td>
              <td class="pr-4">{instruction.instruction.split(" ")[0]}</td>
              <td>{instruction.instruction.split(" ")[1] || ""}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </Show>
  );
};

export default DisassemblyView;
