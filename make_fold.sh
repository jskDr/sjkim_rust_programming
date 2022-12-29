# make executable: chmod +x make_fold.sh
# run: ./make_fold.sh
# run this code by bash shell

# apply_fold=P2_Datastruct_Algorithms_r0
read -p "Input apply fold name: " apply_fold
if [ -z "$apply_fold" ]; then
    apply_fold=P2_Datastruct_Algorithms_r0
    echo "Processng with default fold: $apply_fold"
    exit 1
fi
if[ -d "$apply_fold" ]; then
    cd "$apply_fold"
    for file in *.rs; do
        if [[ "$file" = *_org.rs ]] || [[ "$file" = *_r0.rs ]]; then
            echo "skip $file"
            continue
        fi
        echo "target file = $file"
        foldername="${file%.rs}"
        if [ ! -d "$foldername" ]; then
            if [ -f "$foldername" ]; then
                # this is a file, not a folder
                echo "remove file: $foldername"
                rm "$foldername"
            fi
            echo "generate $foldername"
            mkdir "$foldername"
            for file in "${foldername}"*.*; do
                echo "mv $file $foldername"
                mv "$file" "$foldername"
            done
        fi
        echo ""
    done
    cd ..
fi
