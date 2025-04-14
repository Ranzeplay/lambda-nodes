import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@radix-ui/react-select";
import { Background, Controls, ReactFlow } from "@xyflow/react";
import { Plus } from "lucide-react";
import { useState } from "react";
import FrameView from "~/components/frame";
import ReactFlowWrapper from "~/components/reactflowWrapper";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";

export default function NewPipelinePage() {
	const [name, setName] = useState("");
	const [method, setMethod] = useState("GET");
	const [route, setRoute] = useState("");

	return (
		<FrameView title="Pipelines" subtitle="Create new pipeline">
			<div className="flex flex-col space-y-4">
				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Name</h2>
					<Input type="text" className="border-gray-500" value={name} onChange={(n) => setName(n.target.value)} />
				</div>

				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">Graph</h2>
					<div className="h-[80vh] w-full border border-gray-300 grow">
						<ReactFlow>
							<Background />
							<Controls />
						<Button className="absolute top-2 right-2 z-50 cursor-pointer"><Plus /> Add node</Button>
						</ReactFlow>
					</div>
				</div>

				<div className="space-y-1.5">
					<h2 className="font-bold text-xl">HTTP Route</h2>
					<div className="flex flex-row space-x-4">
						<select title="Method" className="w-[100px] border border-gray-300 rounded-md p-2" value={method} onChange={(n) => setMethod(n.target.value)}>
							<option value="GET">GET</option>
							<option value="POST">POST</option>
							<option value="PUT">PUT</option>
							<option value="DELETE">DELETE</option>
						</select>
						<div className="flex flex-row space-x-1 items-center">
							<p className="font-mono">/pipeline/</p>
							<Input type="text" className="font-mono" value={route} onChange={(n) => setRoute(n.target.value)} />
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
