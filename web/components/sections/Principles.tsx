import Description from "../elements/Description";
import Heading from "../elements/Heading";
import Code from "../icons/Code";
import Lock from "../icons/Lock";
import NetworkNode from "../icons/NetworkNode";
import NoAccount from "../icons/NoAccount";
import SearchX from "../icons/SearchX";
import ThumbUp from "../icons/ThumbUp";

export default function PrinciplesSection() {
    return (
        <section className="min-h-svh max-w-5xl w-fit mx-auto flex flex-col px-8 py-4">
            <Heading>Privacy by design,<br/>not by policy.</Heading>
            <Description>Most services promise not to collect data, but our protocol is designed so 
            that collecting user data is technically impossible. No IP logs, no account 
            databases, no centralized metadata storage — privacy is ensured by 
            architecture, not policy.</Description>

            <div className="flex flex-col space-y-20 py-10">
                <div className="flex flex-row flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <NoAccount/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">No accounts or profiles</h2>
                        <Description>Your email aliases aren't tied to your identity.
                        Your messages aren't tied to your aliases.
                        No account signup required.</Description>
                    </div>
                </div>
                
                <div className="flex flex-row-reverse flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <SearchX/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">Minimal metadata exposure</h2>
                        <Description>Sender and recipient information is encrypted, and messages are routed 
                        through multiple nodes to prevent IP correlation.
                        The network can't see who is talking to whom.</Description>
                    </div>
                </div>

                <div className="flex flex-row flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <NetworkNode/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">Decentralization</h2>
                        <Description>There is no single server controlling your messages. They travel through a 
                        distributed network for reliability, availability, and privacy.</Description>
                    </div>
                </div>
                
                <div className="flex flex-row-reverse flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <Lock/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">Encrypted by default</h2>
                        <Description>Messages are encrypted on your device before they are sent.</Description>
                    </div>
                </div>

                <div className="flex flex-row flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <ThumbUp/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">Easy-to-use</h2>
                        <Description>All cryptography runs under the hood while the interface stays simple and 
                        familiar. Use it like regular email.</Description>
                    </div>
                </div>

                <div className="flex flex-row-reverse flex-wrap space-y-10">
                    <div className="h-30 flex-1/2 flex justify-center items-center text-gray-900 dark:text-primary">
                        <Code/>
                    </div>
                    <div className="flex-1/2 min-w-2xs">
                        <h2 className="text-2xl">Open & Verifiable</h2>
                        <Description>The protocol and code are public, so anyone can verify how the system works.</Description>
                    </div>
                </div>
            </div>
        </section>
    )
}