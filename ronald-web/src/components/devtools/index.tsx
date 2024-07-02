import { Match, Switch, createSignal } from "solid-js";
import CpuStateView from "./CpuStateView";
import DisassemblyView from "./DisassemblyView";
import CrtcStateView from "./CrtcStateView";
import GateArrayStateView from "./GateArrayStateView";
import ScreenStateView from "./ScreenStateView";

type Props = {
  active: boolean;
  snapshot?: any;
  disassembly?: any;
};

const DevTools = (props: Props) => {
  const [activeTab, setActiveTab] = createSignal(0);
  const tabStyles = "mr-1 p-1 rounded-t-md cursor-pointer ";

  return (
    <>
      <ul class="flex">
        <li
          class={
            tabStyles +
            (activeTab() === 0 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(0)}
        >
          Disassembly
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 1 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(1)}
        >
          CPU
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 2 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(2)}
        >
          CRTC
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 3 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(3)}
        >
          Gate Array
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 4 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(4)}
        >
          Screen
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 5 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(5)}
        >
          FDC
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 6 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(6)}
        >
          Tape
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 7 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(7)}
        >
          PPI
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 8 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(8)}
        >
          Keyboard
        </li>
        <li
          class={
            tabStyles +
            (activeTab() === 9 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setActiveTab(9)}
        >
          PSG
        </li>
      </ul>
      <div class="bg-blue-400">
        <Switch>
          <Match when={activeTab() === 0}>
            <DisassemblyView
              active={props.active}
              disassembly={props.disassembly}
            />
          </Match>
          <Match when={activeTab() === 1}>
            <CpuStateView active={props.active} state={props.snapshot?.cpu} />
          </Match>
          <Match when={activeTab() === 2}>
            <CrtcStateView active={props.active} state={props.snapshot?.crtc} />
          </Match>
          <Match when={activeTab() === 3}>
            <GateArrayStateView
              active={props.active}
              state={props.snapshot?.gateArray}
            />
          </Match>
          <Match when={activeTab() === 4}>
            <ScreenStateView
              active={props.active}
              state={props.snapshot?.screen}
            />
          </Match>
        </Switch>
      </div>
    </>
  );
};

export default DevTools;
