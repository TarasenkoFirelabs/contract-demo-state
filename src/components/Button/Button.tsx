import React, { MouseEventHandler } from "react";
import { Colors } from "../../utils/Colors";
import styles from './Button.module.sass';

interface IButtonProps {
    onClick?: MouseEventHandler
    text: any,
    backgroundColor?: any,
    textColor?: any,
    disabled?: any
}

const Button = ({ onClick, text, backgroundColor = 'transparent', textColor = Colors.grey, disabled = false } : IButtonProps) => {
    return (
        <button
            className={ styles.button }
            onClick={ onClick }
            style={{
                backgroundColor: disabled ? Colors.lightGrey: backgroundColor,
                color: disabled ? Colors.grey : textColor
            }}
            disabled={ disabled }
        >
            { text }
        </button>
    )
}

export default Button;
