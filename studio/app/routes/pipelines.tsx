import { Binoculars, Check, LoaderCircle, Pencil, Plus, Rocket } from "lucide-react";
import { Link } from "react-router";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "~/components/ui/tooltip";

export default function PipelinesPage() {
	return (
		<FrameView title="Pipelines" subtitle="Manage pipelines">
			<div className="flex flex-col space-y-4">
				<div className="flex flex-row-reverse">
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
						<TableRow>
							<TableCell className="font-medium">cm9h3toti00003577tgo1c5it</TableCell>
							<TableCell>Demo pipeline</TableCell>
							<TableCell>{new Date().toUTCString()}</TableCell>
							<TableCell>
								<div className="flex flex-row items-center text-blue-600 space-x-2">
									<LoaderCircle className="animate-spin" />
									<p>Running</p>
								</div>
							</TableCell>
							<TableCell className="text-right">
								<div className="flex flex-row justify-end">
									<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
										<TooltipProvider>
											<Tooltip>
												<TooltipTrigger><Pencil /></TooltipTrigger>
												<TooltipContent>
													<p>Edit</p>
												</TooltipContent>
											</Tooltip>
										</TooltipProvider>
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
						<TableRow>
							<TableCell className="font-medium">cm9h42tds00003577rh12ow5t</TableCell>
							<TableCell>Successor</TableCell>
							<TableCell>{new Date().toUTCString()}</TableCell>
							<TableCell>
								<div className="flex flex-row items-center text-green-600 space-x-2">
									<Check />
									<p>Succeeded</p>
								</div>
							</TableCell>
							<TableCell className="text-right">
								<div className="flex flex-row justify-end">
									<Button variant="link" className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
										<TooltipProvider>
											<Tooltip>
												<TooltipTrigger><Pencil /></TooltipTrigger>
												<TooltipContent>
													<p>Edit</p>
												</TooltipContent>
											</Tooltip>
										</TooltipProvider>
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
					</TableBody>
				</Table>
			</div>
		</FrameView>
	);
}
