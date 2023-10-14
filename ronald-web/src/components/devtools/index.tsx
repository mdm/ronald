import { Accessor, Match, Switch, createMemo, createSignal } from "solid-js";
import { Harness } from "../../Harness";
import CpuStateView from "./CpuStateView";
import DisassemblyView from "./DisassemblyView";

type Props = {
    active: boolean;
    harness?: Harness;
};

const DevTools = (props: Props) => {
    const [tab, setTab] = createSignal(0);
    const snapshot = createMemo(() => {
        if (!props.active) {
            return undefined;
        }

        return props.harness?.getSnapshot();
    });

    const tabStyles = "mr-1 p-1 rounded-t-md cursor-pointer ";

    return (
        <>
            <ul class="flex">
                <li
                    class={tabStyles + (tab() === 0 ? "bg-blue-400" : "bg-blue-800 text-blue-400")}
                    onClick={() => setTab(0)}
                >
                    Disassembly
                </li>
                <li class={tabStyles + (tab() === 1 ? "bg-blue-400" : "bg-blue-800 text-blue-400")} onClick={() => setTab(1)}>
                    CPU
                </li>
            </ul>
            <div class="bg-blue-400">
                <Switch>
                    <Match when={tab() === 0}>
                        <DisassemblyView/>
                    </Match>
                    <Match when={tab() === 1}>
                        <CpuStateView state={snapshot()?.cpu}/>
                    </Match>
                </Switch>
            </div>
        </>
    );

};

export default DevTools;
