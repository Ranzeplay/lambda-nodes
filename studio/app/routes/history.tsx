import { Check, LoaderCircle, Wrench, X } from "lucide-react";
import { useEffect, useState } from "react";
import FrameView from "~/components/frame";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { serverAddress } from "~/lib/env";

type History = {
	id: string;
	pipelineId: string;
	status: string;
	startAt: string;
	endAt?: string;
	error?: string;
	result?: any;
  }

export default function HistoryPage() {
	const [histories, setHistories] = useState<History[]>([]);

	useEffect(() => {
		fetch(serverAddress + "/api/history")
			.then((response) => response.json())
			.then((data) => setHistories(data))
			.catch((error) => console.error("Error fetching histories:", error));
	}, []);

	return (
		<FrameView title="History" subtitle="View history">
			<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Id</TableHead>
							<TableHead className="font-bold">Pipeline Id</TableHead>
							<TableHead className="font-bold">Start time</TableHead>
							<TableHead className="font-bold">End time</TableHead>
							<TableHead className="font-bold">Duration</TableHead>
							<TableHead className="font-bold">Status</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{histories.map((history) => (
							<TableRow>
								<TableCell className="font-medium">{history.id}</TableCell>
								<TableCell>{history.pipelineId}</TableCell>
								<TableCell>{new Date(history.startAt).toLocaleString()}</TableCell>
								<TableCell>{history.endAt ? new Date(history.endAt).toLocaleString() : "N/A"}</TableCell>
								<TableCell>{history.endAt ? (new Date(history.endAt).getTime() - new Date(history.startAt).getTime()) + "ms" : "N/A"}</TableCell>
								<TableCell>
									{history.status === "running" && <PipelineRunningState />}
									{history.status === "succeeded" && <PipelineSucceededState />}
									{history.status === "failed" && <PipelineFailedState />}
									{history.status === "preparing" && <PipelinePreparingState />}
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
		</FrameView>
	)
}

const PipelineRunningState = () => {
	return (
		<div className="flex flex-row items-center text-blue-600 space-x-2">
			<LoaderCircle className="animate-spin" />
			<p>Running</p>
		</div>
	);
}

const PipelineSucceededState = () => {
	return (
		<div className="flex flex-row items-center text-green-600 space-x-2">
			<Check />
			<p>Succeeded</p>
		</div>
	);
}

const PipelineFailedState = () => {
	return (
		<div className="flex flex-row items-center text-red-600 space-x-2">
			<X />
			<p>Failed</p>
		</div>
	);
}

const PipelinePreparingState = () => {
	return (
		<div className="flex flex-row items-center text-gray-600 space-x-2">
			<Wrench />
			<p>Preparing</p>
		</div>
	);
}
