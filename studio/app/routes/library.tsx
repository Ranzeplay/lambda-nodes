import { Link } from "react-router";
import FrameView from "~/components/frame";
import { Button } from "~/components/ui/button";
import { TableHeader, TableRow, TableHead, TableBody, TableCell, Table } from "~/components/ui/table";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "~/components/ui/tooltip";
import { Binoculars, Pencil, Plus } from "lucide-react";
import { Input } from "~/components/ui/input";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { fetchAllNodes, type NodeIndexData } from "~/lib/nodeModel";

export default function LibraryPage() {
	const [nodes, setNodes] = useState<NodeIndexData[]>([]);

	useEffect(() => {
		fetchAllNodes()
			.then((data) => {
				setNodes(data!);
			})
			.catch((error) => {
				toast.error("Failed to fetch nodes. Please try again.");
				console.error("Error fetching nodes:", error);
			});
	}, []);

	return (
		<FrameView title="Library" subtitle="All scripts">
			<div className="flex flex-col space-y-4">
				<div className="flex flex-row space-x-4">
					<Input type="text" placeholder="Search..." className="grow" />
					<Button>
						<Link to="/node/new" className="flex items-center gap-x-2 m-0 p-0">
							<Plus /> New
						</Link>
					</Button>
				</div>

				<Table>
					<TableHeader>
						<TableRow>
							<TableHead className="font-bold w-[120px]">Id</TableHead>
							<TableHead className="font-bold">Name</TableHead>
							<TableHead className="font-bold">In data count</TableHead>
							<TableHead className="font-bold">Out data count</TableHead>
							<TableHead className="font-bold text-right pr-6">Operations</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{nodes.map((node) => (
							<TableRow>
								<TableCell className="font-medium">{node.id}</TableCell>
								<TableCell className={`${node.isInternal && 'italic'}`}>{node.name}</TableCell>
								<TableCell>{node.inputs.length}</TableCell>
								<TableCell>{node.outputs.length}</TableCell>
								<TableCell className="text-right">
									{!node.isInternal && (
										<div className="flex flex-row justify-end">
											<Button variant="link">
												<Link to={`/node/edit/${node.id}`} className="text-blue-500 hover:text-blue-700 !m-0 hover:border">
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
										</div>
									)}
								</TableCell>
							</TableRow>
						))}
					</TableBody>
				</Table>
			</div>
		</FrameView>
	);
}
