'use client';
import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";
import { DocumentDuplicateIcon } from "@heroicons/react/20/solid";
import Image from "next/image";

const qr_codes = [
    {
        name: 'bitcoin',
        symbol: 'BTC',
        code: '/btc_qr.png',
        logo: '/btc_logo.png',
        address: 'bc1qq90dh06ah92sg6unfnsn0edx9l9a9msfpagh3f'
    },
    {
        name: 'ethereum',
        symbol: 'ETH',
        code: '/eth_qr.png',
        logo: '/eth_logo.png',
        address: '0xB9be3CbB7Dc9f9C104640899AeF4A1b4147f9e21'
    },
    {
        name: 'solana',
        symbol: 'SOL',
        code: '/sol_qr.png',
        logo: '/sol_logo.png',
        address: '6evKWq8jEVJS1GiaNp7XhKQqZQg4jzacV96KbGLoHLwV'
    },
    {
        name: 'monero',
        symbol: 'XMR',
        code: '/xmr_qr.png',
        logo: '/xmr_logo.png',
        address: '87T3MAroNFfBKE7hvUphVFjfYSgaB6a7qVpvrs9hDAcMMJ413bEeyLAe77j7NnfYeF22PPVbwues9C4Ce2z4N7zv3rXE1Do'
    },
]

export default function() {
    return (
        <div id="donate" className="bg-gray-200 dark:bg-gray-800 border-primary border-2 pt-3 w-md p-5 overflow-hidden rounded-2xl">
            <h2 className="text-3xl pt-3 mb-2">Power Emittio.<br/>Preserve privacy.</h2>
            <p className="mb-3 text-gray-500 dark:text-gray-400">Fund privacy-first infrastructure.</p>
            <TabGroup as="div" vertical className="flex justify-center">
                <TabPanels className="mt-3">
                    {qr_codes.map(e => (
                        <TabPanel key={e.symbol} className="w-full flex flex-col gap-2 items-center">
                            <Image className="dark:invert rounded-xl" src={e.code} alt={`${e.name} qr code`} width={160} height={160}/>
                            <p className="font-mono w-full text-gray-500 dark:text-gray-400">
                                <DocumentDuplicateIcon className="size-5 inline mr-2 cursor-pointer transition-transform active:scale-110" onClick={() => navigator.clipboard.writeText(e.address)}/>
                                {e.address.slice(0, 7)}...{e.address.slice(-7)}
                            </p>
                        </TabPanel>
                    ))}
                </TabPanels>
                <TabList className="flex flex-col gap-1 px-5 justify-center">
                    {qr_codes.map(e => (
                        <Tab key={e.symbol} className="p-2 flex items-center cursor-pointer rounded-2xl justify-center outline-transparent gap-2 focus:not-data-focus:outline-none data-focus:outline border-transparent data-focus:outline-primary data-hover:bg-gray-500/10 dark:data-hover:bg-white/10 transition-colors duration-200 data-selected:border-primary border-2">
                            <Image src={e.logo} width={32} height={32} alt={`${e.name} logo`}/>
                        </Tab>
                    ))}
                </TabList>
            </TabGroup>
        </div>
    )
}