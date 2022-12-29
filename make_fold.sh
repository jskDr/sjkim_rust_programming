# make executable: chmod +x make_fold.sh
# run: ./make_fold.sh
# run this code by bash shell

cd P2_Datastruct_Algorithms_r0
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

