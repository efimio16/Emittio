import JoinWaitlistButton from "./JoinWaitlistButton";
import ThemeToggler from "./ThemeToggler";
import ViewOnGitHub from "./ViewOnGitHub";
import MorphingToggle from "./MorphingTransition";

export default function Header({ showActions = false }: { showActions?: boolean }) {
    return (
        <header className="fixed w-full flex flex-row justify-center p-4 z-1 h-20">
            <MorphingToggle
                blur={24}
                active={showActions ? 1 : 0}
            >{[
                (<div className="
                    w-fit min-w-[33vw] h-16 rounded-full flex justify-center p-4 items-center
                    bg-gray-200/90 text-gray-800
                    dark:bg-gray-800/90 dark:text-gray-200
                ">
                    <img src="/logo-large-light.png" alt="emittio large light logo" className="block dark:hidden h-10 object-contain"/>
                    <img src="/logo-large-dark.png" alt="emittio large dark logo" className="hidden dark:block h-10 object-contain"/>
                </div>),
                (
                    <div className="
                        w-fit min-w-[50vw] gap-3 h-16 rounded-full flex justify-center p-4 items-center
                        bg-gray-200/90 text-gray-800
                        dark:bg-gray-800/90 dark:text-gray-200
                    ">
                        <img src="/logo-small.png" alt="emittio small logo" className="h-10 object-contain"/>
                        <div className="flex-1"/>
                        <div
                            className="flex flex-row overflow-hidden text-sm max-w-fit gap-2"
                        >
                            <ViewOnGitHub small/>
                            <JoinWaitlistButton small/>
                        </div>
                    </div>
                )
            ]}</MorphingToggle>
            <div className="absolute right-4 rounded-full size-12 bg-gray-200/90 dark:bg-gray-800/90">
                <ThemeToggler/>
            </div>
        </header>
    )
}