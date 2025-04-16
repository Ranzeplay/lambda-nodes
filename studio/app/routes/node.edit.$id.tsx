import { useEffect, useState } from "react";
import { useNavigate } from "react-router";
import { toast } from "sonner";
import FrameView from "~/components/frame";
import MonacoEditorWrapper from "~/components/monacoEditorWrapper";
import FlowNode from "~/components/node";
import ReactFlowWrapper from "~/components/reactflowWrapper";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { serverAddress } from "~/lib/env";
import type { NodeData } from "~/lib/nodeModel";

const nodeTypes = { flowNode: FlowNode };

export default function NodeEditPage({ params }: { params: any }) {
	const [name, setName] = useState("");
	const [script, setScript] = useState("function handle({ }) {\n\t// code here\n}\n");
	const [inputs, setInputs] = useState<string[]>([]);
	const [outputs, setOutputs] = useState<string[]>([]);

	const [node, setNode] = useState<{ id: string, type: string, position: { x: number, y: number }, data: NodeData }>
		({
			id: '0',
			type: 'flowNode',
			position: { x: 30, y: 30 },
			data: { name: "", inputs: [], outputs: [], status: "idle" }
		});

	const navigate = useNavigate();
	async function submit() {
		const result = await fetch(serverAddress + "/api/nodes/" + params.id, {
			method: "PUT",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				name,
				script,
				inputs,
				outputs,
			}),
		});

		if (result.ok) {
			navigate("/library");
			toast.success("Node created successfully!");
		} else {
			toast.error("Failed to create node. Please try again.");
		}
	}

	async function deleteCurrent() {
		const result = await fetch(serverAddress + "/api/nodes/" + params.id, {
			method: "DELETE",
			headers: {
				"Content-Type": "application/json",
			},
		});

		if (result.ok) {
			navigate("/library");
			toast.success("Node deleted successfully!");
		} else {
			toast.error("Failed to delete node. Please try again.");
		}
	}

	useEffect(() => {
		fetch(serverAddress + "/api/nodes/" + params.id)
			.then((response) => {
				if (response.ok) {
					return response.json();
				} else {
					throw new Error("Failed to fetch node data");
				}
			})
			.then((data) => {
				setName(data.name);
				setScript(data.script);
				setInputs(data.inputs);
				setOutputs(data.outputs);
			})
			.catch((error) => {
				toast.error("Error fetching node data:", error);
			});
	}, []);

	return (
		<FrameView title="Edit node" subtitle={params.id}>
			<div className="flex flex-col space-y-4">
				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Name</h2>
					<Input type="text" value={name} onChange={(n) => {
						setName(n.target.value);
						const newNode = { ...node };
						newNode.data.name = n.target.value;
						setNode(newNode);
					}} />
				</div>
				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Script</h2>
					<MonacoEditorWrapper height={"56vh"} language="javascript" className="border border-gray-300" value={script} onChange={n => setScript(n ?? "")} />
				</div>
				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Configuration</h2>
					<div className="flex flex-row space-x-4">
						<div className="h-[56vh] border border-gray-300 grow">
							<ReactFlowWrapper nodes={[node]} nodeTypes={nodeTypes} nodesConnectable={false} fitView />
						</div>
						<div className="flex flex-col space-y-2 w-96">
							<div className="flex flex-col space-y-1.5">
								<h3 className="text-lg font-bold">Input</h3>
								{inputs.map((i, index) => (
									<div className="flex flex-row space-x-4" key={index}>
										<Input type="text" className="grow" value={i} onChange={(n) => {
											const newInput = [...inputs];
											newInput[index] = n.target.value;
											setInputs(newInput);

											const newNode = { ...node };
											newNode.data.inputs = newInput;
											setNode(newNode);
										}} />
										<Button variant={"link"} className="block w-min cursor-pointer text-red-500" onClick={() => {
											const newInput = [...inputs];
											newInput.splice(index, 1);
											setInputs(newInput);

											const newNode = { ...node };
											newNode.data.inputs = newInput;
											setNode(newNode);
										}}>Remove</Button>
									</div>
								))}
								<Button variant={"link"} className="block w-min cursor-pointer !text-blue-500" onClick={() => setInputs([...inputs, ""])}>Add</Button>
							</div>
							<div className="flex flex-col space-y-1.5">
								<h3 className="text-lg font-bold">Output</h3>
								{outputs.map((i, index) => (
									<div className="flex flex-row space-x-2" key={index}>
										<Input type="text" className="w-4/5" value={i} onChange={(n) => {
											const newOutput = [...outputs];
											newOutput[index] = n.target.value;
											setOutputs(newOutput);

											const newNode = { ...node };
											newNode.data.outputs = newOutput;
											setNode(newNode);
										}} />
										<Button variant={"link"} className="block w-min cursor-pointer text-red-500" onClick={() => {
											const newOutput = [...outputs];
											newOutput.splice(index, 1);
											setOutputs(newOutput);

											const newNode = { ...node };
											newNode.data.outputs = newOutput;
											setNode(newNode);
										}}>Remove</Button>
									</div>
								))}
								<Button variant={"link"} className="block w-min cursor-pointer !text-blue-500" onClick={() => setOutputs([...outputs, ""])}>Add</Button>
							</div>
						</div>
					</div>

				</div>
				<div className="flex flex-row space-x-4">
					<Button onClick={submit}>Submit</Button>
					<Button variant={"secondary"} onClick={() => navigate("/library")}>Cancel</Button>
					<Button variant={"destructive"} onClick={deleteCurrent}>Delete</Button>
				</div>
			</div>
		</FrameView>
	)
}
