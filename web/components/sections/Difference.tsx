import { 
    BoltIcon,
    BookOpenIcon,
    BriefcaseIcon,
    CheckCircleIcon,
    ClockIcon,
    ExclamationTriangleIcon,
    GlobeAltIcon,
    GlobeAmericasIcon,
    InboxArrowDownIcon,
    LockClosedIcon,
    NewspaperIcon,
    XCircleIcon,
} from "@heroicons/react/20/solid";
import Image from "next/image";

export default function Difference() {
    return (
        <section className="flex flex-col py-4 items-center mb-10">
            <h2 className="text-5xl text-center w-full mb-5">Emittio vs others</h2>
            <div className="min-w-3/4">
                <table className="w-full mb-5">
                    <thead>
                        <tr>
                            <th></th>
                            <th></th>
                            <th className="px-4 py-2"><Image width={30} height={30} alt="gmail logo" src="/gmail.png"/></th>
                            <th className="px-4 py-2"><Image width={30} height={30} alt="protonmail logo" src="/protonmail.png"/></th>
                            <th className="px-4 py-2"><Image width={30} height={30} alt="emittio logo" src="/logo.png"/></th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700">
                            <td className="px-3 py-2"><GlobeAltIcon className="w-10 text-sky-500"/></td>
                            <td className="pr-4 py-2">Decentralized</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700 bg-gray-100 dark:bg-gray-900">
                            <td className="px-3 py-2"><LockClosedIcon className="w-10 text-gray-500"/></td>
                            <td className="pr-4 py-2">End-to-End Encryption</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2 text-nowrap"><CheckCircleIcon className="text-green-600 h-8 inline"/>*</td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700">
                            <td className="px-3 py-2"><NewspaperIcon className="h-10 text-gray-800 dark:text-gray-200"/></td>
                            <td className="pr-4 py-2">Metadata Protection</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><ExclamationTriangleIcon className="text-yellow-400 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700 bg-gray-100 dark:bg-gray-900">
                            <td className="px-3 py-2"><InboxArrowDownIcon className="h-10 text-green-700"/></td>
                            <td className="pr-4 py-2">Anonymous Delivery</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><ExclamationTriangleIcon className="text-yellow-400 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700">
                            <td className="px-3 py-2"><GlobeAmericasIcon className="w-10 text-blue-800"/></td>
                            <td className="pr-4 py-2">Censorship Resistant</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700 bg-gray-100 dark:bg-gray-900">
                            <td className="px-3 py-2"><BookOpenIcon className="h-10 text-orange-300"/></td>
                            <td className="pr-4 py-2">Open Source</td>
                            <td className="px-4 py-2"><XCircleIcon className="text-red-500 h-8 inline"/></td>
                            <td className="px-4 py-2 text-nowrap"><ExclamationTriangleIcon className="text-yellow-400 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700">
                            <td className="px-3 py-2"><BoltIcon className="h-10 text-amber-300"/></td>
                            <td className="pr-4 py-2">Speed</td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                            <td className="px-4 py-2"><ExclamationTriangleIcon className="text-yellow-400 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                        </tr>
                        <tr className="border-b-1 border-b-gray-300 dark:border-b-gray-700 bg-gray-100 dark:bg-gray-900">
                            <td className="px-3 py-2"><BriefcaseIcon className="h-10 text-yellow-900"/></td>
                            <td className="pr-4 py-2">Business Ready</td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                            <td className="px-4 py-2"><CheckCircleIcon className="text-green-600 h-8 inline"/></td>
                            <td className="px-4 py-2"><ClockIcon className="text-gray-500 h-8 inline"/></td>
                        </tr>
                    </tbody>
                </table>
                <div className="text-xs text-gray-700 dark:text-gray-300 mb-5">
                    <p className="mr-5 inline-flex gap-1 items-center">
                        <CheckCircleIcon className="text-green-600 h-6 inline"/>
                        <span>Included</span>
                    </p>
                    <p className="mr-5 inline-flex gap-1 items-center">
                        <ExclamationTriangleIcon className="text-yellow-400 h-6 inline"/>
                        <span>Limited</span>
                    </p>
                    <p className="mr-5 inline-flex gap-1 items-center">
                        <XCircleIcon className="text-red-500 h-6 inline"/>
                        <span>None</span>
                    </p>
                    <p className="mr-5 inline-flex gap-1 items-center">
                        <ClockIcon className="text-gray-500 h-6 inline"/>
                        <span>Planned in future</span>
                    </p>
                </div>
                <p className="italic text-sm text-right text-gray-700 dark:text-gray-300">* ProtonMail encrypts message content, but not headers (sender, recipient, subject)</p>
            </div>
        </section>
    )
}