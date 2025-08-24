import { Index } from "solid-js";
import { hex } from "../../Utils";

type ScreenState = {
  buffer: number[][]; // ignore for UI
  gunPosition: number;
  widthCounter: number;
  waitingForVsync: boolean;
};

type Props = {
  active: boolean;
  state?: ScreenState;
};

const ScreenStateView = (props: Props) => {
  return (
    <>
      <table class="w-full border-separate">
        <tbody>
          <tr>
            <th class="bg-slate-300 text-start">Width Counter</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? props.state!.widthCounter : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Gun Position</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? props.state!.gunPosition : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Waiting for VSYNC</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active
                ? props.state!.waitingForVsync
                  ? "true"
                  : "false"
                : "-"}
            </td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

export default ScreenStateView;
