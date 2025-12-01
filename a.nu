ls day*.rs | get name | each {
    let day = $in | split row "." | get 0;
    mkdir $"($day)/";
    mv  $in $"($day)/mod.rs";
    echo $day;
}
