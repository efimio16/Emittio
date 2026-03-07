import Description from "../elements/Description";
import Heading from "../elements/Heading";
import Subheading from "../elements/Subheading";
import Alias from "../icons/Alias";
import EyeOff from "../icons/EyeOff";
import Incognito from "../icons/Incognito";
import Mailbox from "../icons/Mailbox";
import MailLock from "../icons/MailLock";
import Passkey from "../icons/Passkey";

export default function FeaturesSection() {
    return (
        <section className="mx-auto py-24 px-6 max-w-7xl flex flex-col items-center">
            <Heading>Features</Heading>
            <div className="grid auto-rows-[minmax(200px,auto)] grid-cols-1 gap-6 md:grid-cols-6">
                <FeatureCard className="md:col-span-3 md:row-span-2">
                    <div className="flex-1 flex justify-center items-center p-5 text-orange-400">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <Incognito/>
                        </div>
                    </div>
                    <Subheading>Anonymous email</Subheading>
                    <Description>
                        Send and receive emails without revealing your identity.<br/>
                        No personal data. No account required.
                    </Description>
                </FeatureCard>

                <FeatureCard className="md:col-span-3">
                    <div className="flex-1 flex justify-center items-center p-5 text-cyan-500">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <Alias/>
                        </div>
                    </div>
                    <Subheading>Unlimited aliases</Subheading>
                    <Description>
                        Create as many email addresses as you want — instantly.<br/>
                        Aliases can't be linked back to you or to each other.
                    </Description>
                </FeatureCard>

                <FeatureCard className="md:col-span-3">
                    <div className="flex-1 flex justify-center items-center p-5 text-primary">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <Mailbox/>
                        </div>
                    </div>
                    <Subheading>Works with regular email</Subheading>
                    <Description>
                        Send and receive emails to and from traditional email providers.<br/>
                        No one else needs to install anything.
                    </Description>
                </FeatureCard>

                <FeatureCard className="md:col-span-2">
                    <div className="flex-1 flex justify-center items-center p-5 text-red-400">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <EyeOff/>
                        </div>
                    </div>
                    <Subheading>Block email tracking</Subheading>
                    <Description>
                        Automatically block common tracking techniques used in emails.<br/>
                        No tracking pixels. No hidden requests.
                    </Description>
                </FeatureCard>

                <FeatureCard className="md:col-span-2">
                    <div className="flex-1 flex justify-center items-center p-5 text-gray-400">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <Passkey/>
                        </div>
                    </div>
                    <Subheading>Secure access</Subheading>
                    <Description>
                        Access your inbox using local authentication — without usernames or passwords.<br/>
                        Your identity never leaves your device.
                    </Description>
                </FeatureCard>

                <FeatureCard className="md:col-span-2">
                    <div className="flex-1 flex justify-center items-center p-5 text-yellow-400">
                        <div className="w-fit min-h-30 h-full max-h-60">
                            <MailLock/>
                        </div>
                    </div>
                    <Subheading>PGP support</Subheading>
                    <Description>
                        Use PGP if you want full compatibility with existing encrypted email workflows.<br/>
                        Optional and fully interoperable.
                    </Description>
                </FeatureCard>
            </div>
        </section>
    )
}

function FeatureCard({
    children,
    className = "",
}: {
    children: React.ReactNode;
    className?: string;
}) {
    return (
        <div
            className={`
                group relative rounded-3xl border p-6 flex flex-col
                border-gray-200 bg-white 
                dark:border-gray-800 dark:bg-gray-900
                ${className}
            `}
        >
            {children}

            {/* Placeholder for future illustration
            <div className="pointer-events-none absolute inset-0 opacity-0 transition group-hover:opacity-100">
                {/* future visual layer }
            </div> */}
        </div>
    )
}
