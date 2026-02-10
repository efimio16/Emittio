import clsx from "clsx";

type SubheadingProps = {
    children: React.ReactNode;
    className?: string;
}

export default function Subheading({ children, className }: SubheadingProps) {
    return (
        <h2
            className={clsx(
                "text-2xl mb-5 text-gray-800 dark:text-gray-200",
                className
            )}
        >
            {children}
        </h2>
    );
}