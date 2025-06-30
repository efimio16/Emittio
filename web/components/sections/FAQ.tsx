'use client';
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import { useState } from "react";

export default function() {
    const [open, setOpen] = useState<number>();

    return (
        <section className="flex flex-col py-4 items-center mb-10">
            <h2 className="text-5xl text-center w-full mb-5">FAQ</h2>
            <div className="max-w-300 w-4/5">
                <div data-open={open === 0 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900"  onClick={() => setOpen(open => open === 0 ? undefined : 0)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            How is Emittio different from Gmail or ProtonMail?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Emittio is decentralized, encrypted by default, and doesn't rely on central servers. It uses distributed storage (IPFS), anonymous routing, and requires no personal data or traditional accounts.
                    </p>
                </div>
                <div data-open={open === 1 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 1 ? undefined : 1)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Is Emittio really anonymous?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Yes. Emittio hides your IP, doesn't require personal info, and uses random routes to deliver mail.
                    </p>
                </div>
                <div data-open={open === 2 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 2 ? undefined : 2)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Do I need to register or create an account?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        No traditional signup. You just create a nickname or key — no email, no phone, no ID.
                    </p>
                </div>
                <div data-open={open === 3 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 3 ? undefined : 3)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Is it free to use?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Yes, core features are free. Premium options may come later to support the network.
                    </p>
                </div>
                <div data-open={open === 4 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 4 ? undefined : 4)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Is it open-source?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Yes. All code — clients, nodes, smart contracts — is open and transparent.
                    </p>
                </div>
                <div data-open={open === 5 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 5 ? undefined : 5)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Can anyone run a node?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Yes. Anyone can host a node and help keep the network fast and resilient.
                    </p>
                </div>
                <div data-open={open === 6 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 6 ? undefined : 6)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            What if a node or quorum goes offline?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Your mail is replicated. As long as one node in the quorum is alive, your message survives.
                    </p>
                </div>
                <div data-open={open === 7 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 7 ? undefined : 7)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Can Emittio be censored or blocked?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Very difficult. Since it uses IPFS and independent nodes, there's no central point to target — and messages are cached across multiple regions.
                    </p>
                </div>
                <div data-open={open === 8 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 8 ? undefined : 8)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Is it as fast as traditional email?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Yes. The system is lightweight and optimized for fast delivery without central servers.
                    </p>
                </div>
                <div data-open={open === 9 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 9 ? undefined : 9)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            What data does Emittio store about me?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        None. We don't store IPs, names, emails, or any personal data.
                    </p>
                </div>
                <div data-open={open === 10 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 10 ? undefined : 10)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            Is there a mobile or desktop app?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        A web app is coming first. Mobile and desktop versions will follow soon.
                    </p>
                </div>
                <div data-open={open === 11 || null} className="group mb-3 outline-1 data-open:outline-primary data-open:outline-2 transition-color duration-150 outline-gray-300 rounded-3xl p-5 dark:outline-gray-700 bg-gray-100 dark:bg-gray-900" onClick={() => setOpen(open => open === 11 ? undefined : 11)}>
                    <p className="flex items-center justify-between">
                        <span className="text-lg">
                            How secure is it?
                        </span>
                        <ChevronDownIcon className="size-5 flex-none group-data-open:rotate-180 transition-transform duration-300" />
                    </p>
                    <p className="group-[&:not([data-open])]:hidden text-gray-600 mt-2 dark:text-gray-400">
                        Very. Emittio uses end-to-end encryption, unlinkable routing, and distributed storage — privacy is baked in.
                    </p>
                </div>
            </div>
        </section>
    )
}