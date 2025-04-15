import { Handle, Position } from "@xyflow/react";
import { FunctionSquare } from "lucide-react";
import type { NodeData } from "~/lib/nodeModel";

function getColor(status: NodeData['status']) {
	switch (status) {
		case 'idle':
			return 'bg-gray-500 text-white';
		case 'running':
			return 'bg-blue-500 text-white';
		case 'error':
			return 'bg-red-500 text-white';
		case 'success':
			return 'bg-green-600 text-white';
		default:
			return 'bg-gray-200 text-black';
	}
}

export default function FlowNode({ data, isConnectable }: { data: NodeData, isConnectable: boolean }) {
	return (
		<div className="w-80 border border-gray-300 shadow rounded-md flex flex-col bg-white">
			<div className={`${getColor(data.status)} px-2.5 py-1.5 rounded-t-md flex flex-row relative space-x-2 items-center`}>
				<FunctionSquare />
				<h3>{data.name}</h3>
				<Handle
							type="target"
							position={Position.Left}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-black"
							id={`to-node`}
						/>
				<Handle
							type="source"
							position={Position.Right}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-black"
							id={`from-node`}
						/>
			</div>

			<div>
				{data.inputs.length === 0 ? (
					<h3 className="m-2 font-mono text-sm text-gray-500">No input</h3>
				) : data.inputs.map((name, index) => (
					<div key={index} className="relative flex flex-row space-x-2 items-center px-2 py-1.5">
						<span className="text-gray-700 font-mono">{name}</span>
						<Handle
							type="target"
							position={Position.Left}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-gray-600"
							id={`input-${name}`}
						/>
					</div>
				))}

				<hr className="text-gray-300" />

				{data.outputs.length === 0 ? (
					<h3 className="m-2 font-mono text-sm text-gray-500">No output</h3>
				) : data.outputs.map((name, index) => (
					<div key={index} className="relative flex flex-row space-x-2 items-center px-2 py-1.5">
						<span className="text-gray-700 font-mono">{name}</span>
						<Handle
							type="source"
							position={Position.Right}
							isConnectable={isConnectable}
							className="!w-2.5 !h-2.5 !bg-gray-600"
							id={`output-${name}`}
						/>
					</div>
				))}
			</div>
		</div>

	)
}
