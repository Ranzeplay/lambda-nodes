import FrameView from "~/components/frame";
import { Input } from "~/components/ui/input";
import { Button } from "~/components/ui/button";
import FlowNode from "~/components/node";
import ReactFlowWrapper from "~/components/reactflowWrapper";
import MonacoEditorWrapper from "~/components/monacoEditorWrapper";
import { useState } from "react";

const nodeTypes = { flowNode: FlowNode };

export default function NewNodePage() {
	const [name, setName] = useState("");
	const [script, setScript] = useState("");
	const [input, setInput] = useState<string[]>([]);
	const [output, setOutput] = useState<string[]>([]);

	const [node, setNode] = useState<any>({ id: '0', type: 'flowNode', position: { x: 30, y: 30 }, data: { name: "", input: [], output: [] } });

	return (
		<FrameView title="Nodes" subtitle="Create new node">
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
							<ReactFlowWrapper nodes={[node]} nodeTypes={nodeTypes} />
						</div>
						<div className="flex flex-col space-y-2 grow">
							<div className="flex flex-col space-y-1.5">
								<h3 className="text-lg font-bold">Input</h3>
								{input.map((i, index) => (
									<div className="flex flex-row space-x-2" key={index}>
										<Input type="text" className="w-4/5" value={i} onChange={(n) => {
											const newInput = [...input];
											newInput[index] = n.target.value;
											setInput(newInput);

											const newNode = { ...node };
											newNode.data.input = newInput;
											setNode(newNode);
										}} />
										<Button variant={"link"} className="block w-min cursor-pointer text-red-500" onClick={() => {
											const newInput = [...input];
											newInput.splice(index, 1);
											setInput(newInput);

											const newNode = { ...node };
											newNode.data.input = newInput;
											setNode(newNode);
										}}>Remove</Button>
									</div>
								))}
								<Button variant={"link"} className="block w-min cursor-pointer !text-blue-500" onClick={() => setInput([...input, ""])}>Add</Button>
							</div>
							<div className="flex flex-col space-y-1.5">
								<h3 className="text-lg font-bold">Output</h3>
								{output.map((i, index) => (
									<div className="flex flex-row space-x-2" key={index}>
										<Input type="text" className="w-4/5" value={i} onChange={(n) => {
											const newOutput = [...output];
											newOutput[index] = n.target.value;
											setOutput(newOutput);

											const newNode = { ...node };
											newNode.data.output = newOutput;
											setNode(newNode);
										}} />
										<Button variant={"link"} className="block w-min cursor-pointer text-red-500" onClick={() => {
											const newOutput = [...output];
											newOutput.splice(index, 1);
											setOutput(newOutput);

											const newNode = { ...node };
											newNode.data.output = newOutput;
											setNode(newNode);
										}}>Remove</Button>
									</div>
								))}
								<Button variant={"link"} className="block w-min cursor-pointer !text-blue-500" onClick={() => setOutput([...output, ""])}>Add</Button>
							</div>
						</div>
					</div>

				</div>
				<div className="flex flex-row space-x-4">
					<Button>Submit</Button>
					<Button variant={"secondary"}>Cancel</Button>
				</div>
			</div>
		</FrameView>
	)
}
