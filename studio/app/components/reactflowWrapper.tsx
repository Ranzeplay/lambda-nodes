import { Background, Controls, ReactFlow, type ReactFlowProps } from "@xyflow/react";

import '@xyflow/react/dist/style.css';

export default function ReactFlowWrapper(props: ReactFlowProps) {
	return (
		<ReactFlow {...props}>
			<Background />
			<Controls />
		</ReactFlow>
	)
}