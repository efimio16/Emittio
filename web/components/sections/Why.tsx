import { BoltIcon, GlobeAltIcon, LockClosedIcon, FingerPrintIcon, BookOpenIcon } from "@heroicons/react/20/solid"

export default function Why() {
    return (
        <section className="flex flex-col py-4 mb-10">
            <h2 className="text-5xl text-center w-full mb-5">Why Emittio?</h2>
            <div className="flex flex-row flex-wrap justify-center">
                <div className="w-90 min-w-1/3 flex items-center flex-col text-center m-4 p-4 border-2 border-primary rounded-xl">
                    <LockClosedIcon className="h-20 text-gray-500"/>
                    <h3 className="text-2xl mb-2 text-gray-900 dark:text-gray-200">Encrypted End-to-End</h3>
                    <p className="text-gray-700 dark:text-gray-400">Only sender and recipient can read the message. Not even nodes.</p>
                </div>
                <div className="w-90 min-w-1/3 flex items-center flex-col text-center m-4 p-4 border-2 border-primary rounded-xl">
                    <GlobeAltIcon className="h-20 text-sky-500"/>
                    <h3 className="text-2xl mb-2 text-gray-900 dark:text-gray-200">Decentralized by Design</h3>
                    <p className="text-gray-700 dark:text-gray-400">No servers, no middlemen, no single point of failure.</p>
                </div>
                <div className="w-90 min-w-1/3 flex items-center flex-col text-center m-4 p-4 border-2 border-primary rounded-xl">
                    <FingerPrintIcon className="h-20 text-orange-500"/>
                    <h3 className="text-2xl mb-2 text-gray-900 dark:text-gray-200">No Metadata, No IPs</h3>
                    <p className="text-gray-700 dark:text-gray-400">We never store who you are, where you are, or who you talk to.</p>
                </div>
                <div className="w-90 min-w-1/3 flex items-center flex-col text-center m-4 p-4 border-2 border-primary rounded-xl">
                    <BoltIcon className="h-20 text-amber-300"/> 
                    <h3 className="text-2xl mb-2 text-gray-900 dark:text-gray-200">Fast and Lightweight</h3>
                    <p className="text-gray-700 dark:text-gray-400">Messages route through a network of trusted nodes and IPFS.</p>
                </div>
                <div className="w-90 min-w-1/3 flex items-center flex-col text-center m-4 p-4 border-2 border-primary rounded-xl">
                    <BookOpenIcon className="h-20 text-orange-300"/>
                    <h3 className="text-2xl mb-2 text-gray-900 dark:text-gray-200">Open Source, Auditable</h3>
                    <p className="text-gray-700 dark:text-gray-400">Trust through transparency. Always free, always verifiable.</p>
                </div>
            </div>
        </section>
    )
}