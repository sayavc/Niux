function _niux_packages
    niux -l
end

function _niux_search
    niux --search (commandline -ct)
end

complete -c niux -n "__fish_seen_subcommand_from -l -Hl -Sl -Hr -Sr -Hra -Sra" -f -a "(_niux_packages)"
complete -c niux -n "__fish_seen_subcommand_from -Hi -Si -Hia -Sia" -f -a "(_niux_search)"
