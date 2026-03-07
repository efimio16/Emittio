import { forwardRef } from "react";
import DottedBg from "../elements/DottedBg";
import JoinWaitlistButton from "../elements/JoinWaitlistButton";
import ViewOnGitHub from "../elements/ViewOnGitHub";

const HeroSection = forwardRef<HTMLHRElement, { hideButtons: boolean }>(({ hideButtons }, ref) => {
    return (
        <section className="w-full flex items-center justify-center relative">
            <div className="absolute inset-4 z-0">
                <DottedBg/>
            </div>
            
            <div className="
                min-h-svh min-w-96 w-3/4 relative flex flex-col items-center justify-center z-1 pt-22
                bg-[radial-gradient(white_10%,transparent_70%)]
                dark:bg-[radial-gradient(black_10%,transparent_70%)]
            ">
                <h1 className="text-8xl mb-7 text-center text-gray-800 dark:text-gray-200">Email, without <i>identity</i>.</h1>
                <p className="text-gray-600 p-3 dark:text-gray-500 text-lg mb-5 font-light">
                    A decentralized email network designed for anonymous communication.<br/>
                    Zero-trust by design, fast by architecture, and open by protocol.
                </p>
                <div data-hidden={hideButtons} className="flex flex-wrap relative gap-x-5 gap-y-2 font-light flex-row justify-around items-start blur-none duration-500 transition-[opacity,filter,visibility] ease-out data-[hidden=true]:blur-md data-[hidden=true]:opacity-0 data-[hidden=true]:invisible">
                    <hr ref={ref} className="text-red-500 opacity-0 block absolute w-full -top-15" aria-hidden/>
                    <div className="min-w-60 flex flex-col items-center gap-2">
                        <JoinWaitlistButton/>
                    </div>
                    <div className="min-w-60">
                        <ViewOnGitHub/>
                    </div>
                </div>
            </div>
        </section>
    )
});

export default HeroSection;