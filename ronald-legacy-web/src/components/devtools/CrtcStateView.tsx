import { Index } from "solid-js";
import { hex } from "../../Utils";

type CrtcState = {
  registers: number[];
  selectedRegister: number;
  horizontalCounter: number;
  horizontalSyncWidthCounter: number;
  characterRowCounter: number;
  scanLineCounter: number;
  displayStartAddress: number; // 16-bit
};

type Props = {
  active: boolean;
  state?: CrtcState;
};

const REGISTERS = [
  "Horizontal Total",
  "Horizontal Displayed",
  "Horizontal Sync Position",
  "Horizontal And Vertical Sync Widths",
  "Vertical Total",
  "Vertical Total Adjust",
  "Vertical Displayed",
  "Vertical Sync Position",
  "Interlace And Skew",
  "Maximum Raster Address",
  "Cursor Start Raster",
  "Cursor End Raster",
  "Display Start Address High",
  "Display Start Address Low",
  "Cursor Address High",
  "Cursor Address Low",
  "Light Pen Address High",
  "Light Pen Address Low",
];

const CrtcStateView = (props: Props) => {
  return (
    <>
      Registers
      <table class="w-full border-separate">
        <tbody>
          <Index each={props.state!.registers}>
            {(registerValue, index) => (
              <tr>
                <th class="bg-slate-300 text-start">{REGISTERS[index]}</th>
                <td
                  class={
                    props.active && props.state!.selectedRegister === index
                      ? "bg-blue-800 text-blue-400 border-2 border-slate-100 text-center w-10"
                      : "bg-slate-100 text-center w-10"
                  }
                >
                  {props.active ? hex(registerValue(), 2) : "-"}
                </td>
              </tr>
            )}
          </Index>
        </tbody>
      </table>
      Counters
      <table class="w-full border-separate">
        <tbody>
          <tr>
            <th class="bg-slate-300 text-start">Horizontal Counter</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.horizontalCounter, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">
              Horizontal Sync Width Counter
            </th>
            <td class="bg-slate-100 text-center w-10">
              {props.active
                ? hex(props.state!.horizontalSyncWidthCounter, 2)
                : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Character Row Counter</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.characterRowCounter, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Scan Line Counter</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.scanLineCounter, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">
              Display Start Address (copy)
            </th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.displayStartAddress, 4) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

export default CrtcStateView;
