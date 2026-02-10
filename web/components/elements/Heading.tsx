import clsx from "clsx";

type HeadingProps = {
    children: React.ReactNode;
    className?: string;
}

export default function Heading({ children, className }: HeadingProps) {
    return (
        <h1
            className={clsx(
                "text-6xl mb-7 text-gray-800 dark:text-gray-200",
                className
            )}
        >
            {children}
        </h1>
    );
}