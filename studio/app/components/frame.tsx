import { Home, Inbox, Calendar, Search, Settings, LayoutDashboard, Code, PenBox, Logs } from "lucide-react";
import { Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider, SidebarTrigger } from "./ui/sidebar";

const items = [
	{
		title: "Dashboard",
		url: "#",
		icon: LayoutDashboard,
	},
	{
		title: "Library",
		url: "#",
		icon: Code,
	},
	{
		title: "Editor",
		url: "#",
		icon: PenBox,
	},
	{
		title: "Logs",
		url: "#",
		icon: Logs,
	},
	{
		title: "Settings",
		url: "#",
		icon: Settings,
	},
]

export default function FrameView({ children, title, subtitle }: { children: React.ReactNode, title: string, subtitle: string }) {
	return (
		<SidebarProvider>
			<Sidebar className="bg-gray-50 border-r border-gray-200 shadow p-3">
				<SidebarContent>
					<SidebarGroup className="space-y-4">
						<SidebarGroupLabel className="text-xl font-bold font-serif">Lambda Nodes</SidebarGroupLabel>
						<SidebarGroupContent>
							<SidebarMenu className="space-y-1">
								{items.map((item) => (
									<SidebarMenuItem key={item.title} className="hover:bg-gray-100 hover:shadow rounded-md transition">
										<SidebarMenuButton asChild>
											<a href={item.url}>
												<item.icon className="text-gray-400" />
												<span>{item.title}</span>
											</a>
										</SidebarMenuButton>
									</SidebarMenuItem>
								))}
							</SidebarMenu>
						</SidebarGroupContent>
					</SidebarGroup>
				</SidebarContent>
			</Sidebar>
			<main className="p-6 w-full flex flex-col gap-y-2">
				{/* <SidebarTrigger /> */}
				<div className="w-full">
					<h1 className="text-2xl font-bold">{title}</h1>
					<p className="text-gray-500">{subtitle}</p>
					<hr className="my-4 text-gray-300" />
				</div>
				{children}
			</main>
		</SidebarProvider>
	);
}
