import { Index } from "solid-js";
import { hex } from "../../Utils";

type TrackState = {};

type DiskState = {};

type DriveState = {
  track: number;
  sector: number;
  disk: DiskState | null;
};

type FdcState = {
  drives: DriveState[];
  phase: string;
  command: string | null;
  parametersBuffer: number[];
  dataBuffer: number[];
  resultBuffer: number[];
  motorsOn: boolean;
  requestForMaster: boolean;
  dataInputOutput: boolean; // false: CPU -> FDC, true: FDC -> CPU
  executionMode: boolean;
  floppyControllerBusy: boolean;
  floppyDriveBusy: boolean[]; // TODO: do we need this? commands finish immediately in our implementation
  seekEnd: boolean;
  driveNotReady: boolean;
  selectedDrive: number;
  endOfTrack: boolean;
  status1: number;
  status2: number;
};

type Props = {
  active: boolean;
  state?: FdcState;
};

const FdcStateView = (props: Props) => {
  return (
    <>
      <table class="w-full border-separate">
        <tbody>
          <tr>
            <th class="bg-slate-300 text-start">Phase</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active ? props.state!.phase : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Command</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active && props.state!.command
                ? props.state!.command
                : "-"}
            </td>
          </tr>
          <tr>
            <th class="bg-slate-300 text-start">Parameters Buffer</th>
            <td class="bg-slate-100 text-center w-10">
              {props.active
                ? props.state!.parametersBuffer.map((b) => hex(b)).join(" ")
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

export default FdcStateView;
