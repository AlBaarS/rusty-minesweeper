import classNames from "classnames";

export function getButtonMarkup(colour: string) {
    return(
        classNames(
            "text-black",
            "border-4",
            "[border-style:outset]",
            "border-gray-400",
            "px-7",
            "pb-[8px]",
            "pt-[10px]",
            "text-sm",
            "font-medium",
            "uppercase",
            "leading-normal",
            "transition duration-150",
            "ease-in-out",
            `hover:border-${colour}-400`,
            `hover:bg-${colour}-400`,
            "hover:text-neutral-100",
            "disabled:border-neutral-100",
            "disabled:text-neutral-100",
            "disabled:bg-neutral-300",
            "w-full",
        )
    );
};

export default getButtonMarkup;