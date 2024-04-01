import { Match, Switch, createSignal } from "solid-js";
import CpuStateView from "./CpuStateView";
import DisassemblyView from "./DisassemblyView";

type Props = {
  active: boolean;
  snapshot?: any;
  disassembly?: any;
};

const DevTools = (props: Props) => {
  const [tab, setTab] = createSignal(0);
  const tabStyles = "mr-1 p-1 rounded-t-md cursor-pointer ";

  return (
    <>
      <ul class="flex">
        <li
          class={
            tabStyles +
            (tab() === 0 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setTab(0)}
        >
          Disassembly
        </li>
        <li
          class={
            tabStyles +
            (tab() === 1 ? "bg-blue-400" : "bg-blue-800 text-blue-400")
          }
          onClick={() => setTab(1)}
        >
          CPU
        </li>
      </ul>
      <div class="bg-blue-400">
        <Switch>
          <Match when={tab() === 0}>
            <DisassemblyView
              active={props.active}
              disassembly={props.disassembly}
            />
          </Match>
          <Match when={tab() === 1}>
            <CpuStateView active={props.active} state={props.snapshot?.cpu} />
          </Match>
        </Switch>
      </div>
    </>
  );
};

export default DevTools;
