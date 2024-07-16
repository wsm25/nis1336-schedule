import { FlagTwoTone } from "@ant-design/icons";

export const priColor={
    default: "#888888",
    low: "#4772fa",
    mid: "#faa80c",
    high: "#d52b24",
};

export var priFlags={}
for (let key in priColor){
    priFlags[key]=<FlagTwoTone twoToneColor={priColor[key]}/>
}