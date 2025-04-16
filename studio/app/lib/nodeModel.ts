import { serverAddress } from "./env";

export type NodeData = {
  name: string;
  inputs: string[];
  outputs: string[];
  script?: string;
  status: "idle" | "running" | "error" | "success";
};

export type NodeIndexData = NodeData & {
	id: string;
	isInternal: boolean;
};

export async function fetchAllNodes() {
  const response = await fetch(serverAddress + "/api/nodes");
	if (response.ok) {
    return response.json() as Promise<NodeIndexData[]>;
  } else {
    Promise.reject(new Error("Failed to fetch nodes"));
  }
}
