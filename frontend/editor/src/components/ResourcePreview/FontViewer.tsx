import React, { useEffect, useMemo, useRef, useState } from "react";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";

const FontViewer = (props) => {
    const { name, path, file_type, last_modified_time } = props;

    const [font , setFont] = useState();

    const id = useMemo(() => {
        return `${name}-${file_type}`;
    }, [name, file_type]);


    useEffect(() => {
        invoke("parse_font_metadata", { path })
            .then((res) => {
                console.log(res);
                setFont(res);
                const url = convertFileSrc(path);
                const fontFace = new FontFace(res.full_name, `url(${url})`);
                fontFace
                    .load()
                    .then((loadedFont) => {
                        document.fonts.add(loadedFont);
                        console.log(`字体 ${res.full_name} 加载完成`);
                    })
                    .catch((error) => console.error(`加载字体 ${res.full_name} 失败:`, error));
            })
            .catch((error) => {
                console.log(error);
            });
    }, []);

    if (!font) {
        return null;
    }

    return (
        <div key={font.full_name} style={{ fontFamily: font.full_name }}>
         <p className="text-xl">汉</p>
          <p className="text-xl">{font.full_name}</p>
        </div>
    )
};

export default FontViewer;
