import { addEdge, Background, Controls, ReactFlow, useEdgesState, useNodesState } from "@xyflow/react";
import { Plus } from "lucide-react";
import { useCallback, useEffect, useId, useState } from "react";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from "~/components/ui/select";

import '@xyflow/react/dist/style.css';
import FlowNode from "~/components/node";
import { fetchAllNodes, type NodeIndexData } from "~/lib/nodeModel";
import { toast } from "sonner";
import { DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose, Drawer } from "~/components/ui/drawer";
import { v4 as uuidv4 } from 'uuid';
import { useNavigate } from "react-router";
import { serverAddress } from "~/lib/env";

const nodeTypes = { flowNode: FlowNode };

export default function NewPipelinePage() {
	const [name, setName] = useState("");
	const [method, setMethod] = useState("GET");
	const [route, setRoute] = useState("");

	const [nodes, setNodes, onNodesChange] = useNodesState([]);
	const [edges, setEdges, onEdgesChange] = useEdgesState([]);

	const [nodeAditionDrawerOpen, setNodeAditionDrawerOpen] = useState(false);

	const [availableNodes, setAvailableNodes] = useState<NodeIndexData[]>();
	useEffect(() => {
		fetchAllNodes()
			.then((nodes) => {
				setAvailableNodes(nodes);
			})
			.catch((error) => {
				toast.error("Error fetching nodes:", error);
			});
	}, []);

	const onConnect = useCallback(
		// @ts-ignore
		(connection: any) => setEdges((eds: any) => addEdge(connection, eds)),
		[setEdges],
	);

	function addNode(nodeId: string) {
		const node = availableNodes?.find((n) => n.id === nodeId)!;

		const newNode = {
			id: uuidv4(),
			type: 'flowNode',
			position: { x: 0, y: 0 },
			data: {
				id: node.id,
				name: node.name,
				inputs: node.inputs,
				outputs: node.outputs,
				status: node.status,
			},
		};
		// @ts-ignore
		setNodes((nds) => nds.concat(newNode));
		toast.success(`Node ${node.name} added successfully!`);

		setNodeAditionDrawerOpen(false);
	}

	async function submit() {
		const result = await fetch(serverAddress + "/api/pipelines", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name,
				method,
				url: route,
				content: {
					nodes,
					edges
				}
			}),
		});

		if (result.ok) {
			toast.success("Pipeline created successfully!");
			navigate("/pipelines");
		} else {
			toast.error("Failed to create pipeline. Please try again.");
		}
	}

	const navigate = useNavigate();

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
							<Drawer open={nodeAditionDrawerOpen} onOpenChange={setNodeAditionDrawerOpen} direction="right">
								<DrawerTrigger>
									<Button className="absolute top-2 right-2 z-50 cursor-pointer"><Plus /> Add node</Button>
								</DrawerTrigger>
								<DrawerContent>
									<DrawerHeader>
										<DrawerTitle>Select node</DrawerTitle>
										<DrawerDescription>{availableNodes?.length} node(s) in total</DrawerDescription>
									</DrawerHeader>
									<div className="flex flex-col space-y-1 mx-3.5">
										<Input type="text" placeholder="Search node..." className="mb-2" />
										{availableNodes?.map((node, index) => (
											<Button key={index} variant="link" className="block w-min text-blue-500" onClick={() => addNode(node.id)}>{node.name}</Button>
										))}
									</div>
									<DrawerFooter className="flex flex-row-reverse">
										<DrawerClose className="flex">
											<Button variant="outline">Cancel</Button>
										</DrawerClose>
									</DrawerFooter>
								</DrawerContent>
							</Drawer>
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
							<p className="font-mono">/exec/</p>
							<Input type="text" className="font-mono grow" value={route} onChange={(n) => setRoute(n.target.value)} />
						</div>
					</div>
				</div>

				<div className="flex flex-row space-x-4">
					<Button onClick={submit}>Submit</Button>
					<Button variant={"secondary"} onClick={() => navigate("/pipelines")}>Cancel</Button>
				</div>
			</div>
		</FrameView>
	);
}
