import { Binoculars, Check, Dot, LoaderCircle, Pencil, Plus, Rocket } from "lucide-react";
import { useEffect, useState } from "react";
import { Link } from "react-router";
import { toast } from "sonner";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "~/components/ui/tooltip";
import { serverAddress } from "~/lib/env";

export default function PipelinesPage() {
	const [pipelines, setPipelines] = useState<any[]>([]);

	useEffect(() => {
		// Fetch pipelines from the API or any other source
		fetch(serverAddress + "/api/pipelines")
			.then((response) => response.json())
			.then((data) => setPipelines(data))
			.catch((error) => toast.error("Error fetching pipelines:", error));
	}, []);

	return (
		<FrameView title="Pipelines" subtitle="Manage pipelines">
			<div className="flex flex-col space-y-4">
				<div className="flex flex-row space-x-4">
					<Input type="text" placeholder="Search..." className="grow" />
					<Button>
						<Link to="/pipeline/new" className="flex items-center gap-x-2 m-0 p-0">
							<Plus /> New
						</Link>
					</Button>
				</div>

				<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Id</TableHead>
							<TableHead className="font-bold">Name</TableHead>
							<TableHead className="font-bold">Last run</TableHead>
							<TableHead className="font-bold">Status</TableHead>
							<TableHead className="font-bold text-right pr-6">Operations</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{pipelines.map((pipeline) => (
							<TableRow>
								<TableCell className="font-medium">{pipeline.id}</TableCell>
								<TableCell>{pipeline.name}</TableCell>
								<TableCell>N/A</TableCell>
								<TableCell>
									<PipelineReadyState />
								</TableCell>
								<TableCell className="text-right">
									<div className="flex flex-row justify-end">
										<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
											<Link to={`/pipeline/edit/${pipeline.id}`}>
												<TooltipProvider>
													<Tooltip>
														<TooltipTrigger><Pencil /></TooltipTrigger>
														<TooltipContent>
															<p>Edit</p>
														</TooltipContent>
													</Tooltip>
												</TooltipProvider>
											</Link>
										</Button>
										<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
											<TooltipProvider>
												<Tooltip>
													<TooltipTrigger><Binoculars /></TooltipTrigger>
													<TooltipContent>
														<p>Inspect</p>
													</TooltipContent>
												</Tooltip>
											</TooltipProvider>
										</Button>
										<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
											<TooltipProvider>
												<Tooltip>
													<TooltipTrigger><Rocket /></TooltipTrigger>
													<TooltipContent>
														<p>Manual Run</p>
													</TooltipContent>
												</Tooltip>
											</TooltipProvider>
										</Button>
									</div>
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</div>
		</FrameView>
	);
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
			<Check />
			<p>Failed</p>
		</div>
	);
}

const PipelineReadyState = () => {
	return (
		<div className="flex flex-row items-center text-gray-600 space-x-2">
			<Dot />
			<p>Ready</p>
		</div>
	);
}
