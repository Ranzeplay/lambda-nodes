import { Binoculars, Check, LoaderCircle, Wrench, X } from "lucide-react";
import { useEffect, useState } from "react";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle, DrawerTrigger } from "~/components/ui/drawer";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "~/components/ui/tooltip";
import { serverAddress } from "~/lib/env";
import MonacoEditorWrapper from "~/components/monacoEditorWrapper";

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
						<TableHead className="font-bold text-right pr-6">Operations</TableHead>
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
								<TableCell className="text-right">
									<Drawer direction="right">
										<DrawerTrigger asChild>
											<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border *:cursor-pointer">
												<TooltipProvider>
													<Tooltip>
														<TooltipTrigger><Binoculars /></TooltipTrigger>
														<TooltipContent>
															<p>Inspect</p>
														</TooltipContent>
													</Tooltip>
												</TooltipProvider>
											</Button>
										</DrawerTrigger>
										<DrawerContent>
											<DrawerHeader>
												<DrawerTitle>Inspect pipeline history</DrawerTitle>
												<DrawerDescription>{history.id}</DrawerDescription>
											</DrawerHeader>

											<div className="flex flex-col space-y-2 mx-4">
												<hr className="border border-gray-300" />
												<div className="flex flex-col space-y-1">
													<h3 className="font-semibold">Duration</h3>
													<p className="font-mono ml-2 text-gray-600">{history.endAt ? (new Date(history.endAt).getTime() - new Date(history.startAt).getTime()) + "ms" : "N/A"}</p>
												</div>
												<div className="flex flex-col space-y-1">
													<h3 className="font-semibold">Error</h3>
													<MonacoEditorWrapper 
														value={history.error ?? "<unknown>"}
														language="json"
														options={{
															readOnly: true,
															minimap: { enabled: false },
															scrollBeyondLastLine: false,
															automaticLayout: true,
														}}
														height={"18vh"}
														className="border border-gray-300"
													/>
												</div>
												<div className="flex flex-col space-y-1">
													<h3 className="font-semibold">Result</h3>
													<MonacoEditorWrapper 
														value={history.result ?? "<unknown>"}
														language="json"
														options={{
															readOnly: true,
															minimap: { enabled: false },
															scrollBeyondLastLine: false,
															automaticLayout: true,
														}}
														height={"18vh"}
														className="border border-gray-300"
													/>
												</div>
											</div>
											<DrawerFooter className="flex flex-row-reverse">
												<DrawerClose className="flex">
													<Button variant="outline">Close</Button>
												</DrawerClose>
											</DrawerFooter>
										</DrawerContent>
									</Drawer>
								</TableCell>
							</TableRow>
						)
					)}
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
