import { addEdge, Background, Controls, ReactFlow, useEdgesState, useNodesState } from "@xyflow/react";
import { Plus } from "lucide-react";
import { useCallback, useState } from "react";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "~/components/ui/select";

import '@xyflow/react/dist/style.css';
import FlowNode from "~/components/node";

const nodeTypes = { flowNode: FlowNode };

export default function NewPipelinePage() {
	const [name, setName] = useState("");
	const [method, setMethod] = useState("GET");
	const [route, setRoute] = useState("");

	const [nodes, setNodes, onNodesChange] = useNodesState([
		{ id: '1', type: 'flowNode', position: { x: 0, y: 0 }, data: { name: "Begin", input: [], output: ["out1"] }, status: "idle" },
		{ id: '-1', type: 'flowNode', position: { x: 20, y: 20 }, data: { name: "End", input: ["in"], output: [] }, status: "fail" },
		{ id: '10', type: 'flowNode', position: { x: 40, y: 40 }, data: { name: "Intermediate 1", input: ["in1"], output: ["out1"], status: "running" } },
		{ id: '20', type: 'flowNode', position: { x: 60, y: 60 }, data: { name: "Intermediate 2", input: ["in1", "in2"], output: ["out1"], status: "success" } },
	]);
	const [edges, setEdges, onEdgesChange] = useEdgesState([]);

	const onConnect = useCallback(
		(connection) => setEdges((eds) => addEdge(connection, eds)),
		[setEdges],
	);

	return (
		<FrameView title="Pipelines" subtitle="Create new pipeline">
			<div className="flex flex-col space-y-4">
				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Name</h2>
					<Input type="text" value={name} onChange={(n) => setName(n.target.value)} />
				</div>

				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Graph</h2>
					<div className="h-[80vh] w-full border border-gray-300 grow">
						<ReactFlow nodeTypes={nodeTypes} nodes={nodes} edges={edges} onNodesChange={onNodesChange} onEdgesChange={onEdgesChange} onConnect={onConnect} fitView>
							<Background />
							<Controls />
							<Button className="absolute top-2 right-2 z-50 cursor-pointer"><Plus /> Add node</Button>
						</ReactFlow>
					</div>
				</div>

				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">HTTP Route</h2>
					<div className="flex flex-row space-x-4">
						<Select>
							<SelectTrigger className="w-[210px]">
								<SelectValue placeholder="Select a HTTP method" />
							</SelectTrigger>
							<SelectContent>
								<SelectGroup>
									<SelectLabel>HTTP Methods</SelectLabel>
									<SelectItem value="GET">GET</SelectItem>
									<SelectItem value="POST">POST</SelectItem>
									<SelectItem value="PUT">PUT</SelectItem>
									<SelectItem value="DELETE">DELETE</SelectItem>
									<SelectItem value="PATCH">PATCH</SelectItem>
								</SelectGroup>
							</SelectContent>
						</Select>
						<div className="flex flex-row space-x-1 items-center grow">
							<p className="font-mono">/pipeline/</p>
							<Input type="text" className="font-mono grow" value={route} onChange={(n) => setRoute(n.target.value)} />
						</div>
					</div>
				</div>

				<div className="flex flex-row space-x-4">
					<Button>Submit</Button>
					<Button variant={"secondary"}>Cancel</Button>
				</div>
			</div>
		</FrameView>
	);
}
