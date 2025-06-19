import { Index } from "solid-js";
import { hex } from "../../Utils";

type GateArrayState = {
  currentScreenMode: number;
  requestedScreenMode: number;
  hsyncActive: boolean;
  vsyncActive: boolean;
  hsyncsSinceLastVsync: number;
  interruptCounter: number;
  selectedPen: number;
  penColors: number[];
};

type Props = {
  active: boolean;
  state?: GateArrayState;
};

const GateArrayStateView = (props: Props) => {
  return (
    <>
      <table class="w-full border-separate">
        <tbody>
          <tr>
            <th class="bg-slate-300 text-start">Current Screen Mode</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.currentScreenMode, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Requested Screen Mode</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.requestedScreenMode, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">HSYNC Active</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active
                ? props.state!.hsyncActive
                  ? "true"
                  : "false"
                : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">VSYNC Active</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active
                ? props.state!.vsyncActive
                  ? "true"
                  : "false"
                : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">HSYNCs Since Last VSYNC</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.hsyncsSinceLastVsync, 2) : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Interrupt Counter</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? hex(props.state!.interruptCounter, 2) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
      <table class="w-full border-separate">
        <thead>
          <tr>
            <th class="bg-slate-300 text-start" colSpan="8">
              Pen Colors
            </th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <Index each={props.state!.penColors.slice(0, 8)}>
              {(color, index) => (
                <td
                  class={
                    props.state!.selectedPen == index
                      ? "bg-blue-800 text-blue-400 border-2 border-slate-100 text-center w-10"
                      : "bg-slate-100 text-center w-10"
                  }
                >
                  {props.active ? hex(color(), 2) : "-"}
                </td>
              )}
            </Index>
          </tr>
          <tr>
            <Index each={props.state!.penColors.slice(8, 16)}>
              {(color, index) => (
                <td
                  class={
                    props.state!.selectedPen == 8 + index
                      ? "bg-blue-800 text-blue-400 border-2 border-slate-100 text-center w-10"
                      : "bg-slate-100 text-center w-10"
                  }
                >
                  {props.active ? hex(color(), 2) : "-"}
                </td>
              )}
            </Index>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start" colSpan="3">
              Border
            </th>
            <td class="bg-slate-100 text-center w-10" colSpan="5">
              {props.active ? hex(props.state!.penColors[0x10], 2) : "-"}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

export default GateArrayStateView;
