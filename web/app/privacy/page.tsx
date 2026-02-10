import Description from "@/components/elements/Description";
import Footer from "@/components/elements/Footer";
import Header from "@/components/elements/Header";
import Heading from "@/components/elements/Heading";
import Subheading from "@/components/elements/Subheading";
import ViewOnGitHub from "@/components/elements/ViewOnGitHub";
import FirstCta from "@/components/sections/FirstCta";

export default function Privacy() {
    return (
        <>
            <Header showActions/>
            <main className="px-4 mt-28 max-w-7xl mx-auto min-h-svh">
                <Heading>Privacy by design</Heading>
                <Description>
                    Your privacy is built into the system from the ground up.<br/>
                    We don't just promise not to collect data — our architecture makes it technically impossible.
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>What we don't collect</Subheading>
                <Description>
                    - No account registrations or profiles<br/>
                    - No IP logs<br/>
                    - No centralized storage of messages or metadata<br/>
                    - Your email aliases cannot be linked to you
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>Message retention</Subheading>
                <Description>
                    Messages are stored temporarily in the network for up to one week.<br/>
                    This is done to prevent the network from being overloaded by large volumes of temporary emails, newsletters, and verification codes.<br/>
                    You can still optionally save emails locally on your device for as long as you like, giving you full control over your personal archive while keeping the network fast and efficient.
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>Encryption</Subheading>
                <Description>
                    All messages are encrypted on your device before leaving it.<br/>
                    Only the intended recipient can decrypt and read them.
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>Aliases & account-less design</Subheading>
                <Description>
                    - Create multiple email aliases locally<br/>
                    - Aliases are unlinkable and not tied to your identity<br/>
                    - No usernames or passwords required
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>Recovery</Subheading>
                <Description>
                    You can download a recovery file containing your private seed to restore your aliases if devices are lost.<br/>
                    Important:<br/>
                    1. This is the only way to recover your aliases<br/>
                    2. Keep it secure and offline — anyone with the seed has full access to your inbox
                </Description>
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <Subheading>Open & Verifiable</Subheading>
                <Description>
                    The protocol and code are public.
                    Anyone can inspect how the system works and verify privacy protections.
                    <ViewOnGitHub/>
                </Description>  
                <hr className="text-gray-200 dark:text-gray-800 my-10"/>

                <FirstCta/>
            </main>
            <Footer/>
        </>
    )
}
