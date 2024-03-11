
#let ref-enzyme = [@NEURIPS2020_9332c513 #ref(label("10.1145/3458817.3476165")) #ref(label("10.5555/3571885.3571964"))]

#let diff-package = link("https://github.com/LucaCiucci/differential-rs")[`differential`] + [@differential-repo]

#let typ_label = label
#let appendix(title, body, render: (it, label) => it, label: "") = {
    let h = heading([Appendix: #title], supplement: "Appendix");
    if label != "" {
        [#h #typ_label(label)]
    } else {
        h
    }
    render(body, label)
}

#let branch = {
  let file = read("../.git/HEAD");
  let branch = file.split("/").last();
  branch.trim()
}
#let orig_hash = {
  let file = read("../.git/refs/heads/" + branch);
  file.trim()
}

// example https://github.com/LucaCiucci/differential-rs/blob/d0e5265b9a68b21b916f803d1ced764996c279a8/Cargo.toml#L13
#let code_ref(file, line: -1) = {
  let url = "https://github.com/LucaCiucci/differential-rs/blob/" + orig_hash + "/" + file;
  let line = if type(line) == "string" {
    let file = read("../" + file);
    let line = file.split("\n").enumerate().find((nl) => nl.at(1).contains(line));
    if line == none {
      panic("line not found")
    }
    line.at(0) + 1
  } else {
    line
  }
  let content = if line < 0 {
    file
  } else {
    url = url + "#L" + str(line);
    file + ":" + str(line)
  };
  link(url, raw(content, block: false))
  footnote(link(url, url))
}