import type { Component, JSX } from "solid-js";

type Props = {
  title?: string;
  children?: JSX.Element;
  onClick?: (event: MouseEvent) => void;
};

const ControlButton = (props: Props) => {
  return (
    <button
      title={props.title}
      class="w-10 h-10 flex justify-center items-center m-2 rounded text-slate-900 bg-slate-400 hover:bg-slate-200 flex-grow-0 text-sm"
      onClick={props.onClick}
    >
      {props.children}
    </button>
  );
};

export default ControlButton;
