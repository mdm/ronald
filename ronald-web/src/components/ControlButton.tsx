import type { Component, JSX} from "solid-js";

type Props = {
  children?: JSX.Element;
  onClick?: (event: MouseEvent) => void;
}

const ControlButton = (props: Props) => {
  return (
    <button
      class="w-20 h-20 m-2 rounded text-slate-900 bg-slate-400 hover:bg-slate-200 flex-grow-0 text-sm"
      onClick={props.onClick}
    >
      {props.children}
    </button>
  );
};

export default ControlButton;
