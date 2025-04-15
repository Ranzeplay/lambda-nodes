export type NodeData = {
  name: string;
  inputs: string[];
  outputs: string[];
  status: "idle" | "running" | "error" | "success";
};
