import { Handle, Position } from "@xyflow/react";
import { FunctionSquare } from "lucide-react";

export type NodeData = {
	name: string,
	input: string[],
	output: string[],
};

export default function FlowNode({ data, isConnectable }: { data: NodeData, isConnectable: boolean }) {
	return (
		<div className="w-64 border border-gray-300 shadow rounded-md flex flex-col bg-white">
			<div className="bg-blue-600 text-white px-2.5 py-1.5 rounded-t-md flex flex-row space-x-2 items-center">
				<FunctionSquare />
				<h3>{data.name}</h3>
			</div>

			<div>
				{data.input.length === 0 ? (
					<h3 className="m-2 font-mono text-sm text-gray-500">No input</h3>
				) : data.input.map((i, index) => (
					<div key={index} className="relative flex flex-row space-x-2 items-center px-2 py-1.5 border-b border-gray-300">
						<span className="text-gray-500 font-mono">{i}</span>
						<Handle
							type="target"
							position={Position.Left}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-black"
							id={`input-${index}`}
						/>
					</div>
				))}

				<hr className="text-gray-300" />

				{data.output.length === 0 ? (
					<h3 className="m-2 font-mono text-sm text-gray-500">No output</h3>
				) : data.output.map((o, index) => (
					<div key={index} className="relative flex flex-row space-x-2 items-center px-2 py-1.5 border-b border-gray-300">
						<span className="text-gray-500 font-mono">{o}</span>
						<Handle
							type="source"
							position={Position.Right}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-black"
							id={`output-${index}`}
						/>
					</div>
				))}
			</div>
		</div>

	)
}
