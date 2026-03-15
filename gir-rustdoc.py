#!/usr/bin/env python3

from glob import iglob
from itertools import chain
from os import getenv, getcwd, makedirs, chdir, symlink
from shlex import quote
from string import Template
import argparse
import datetime
import gzip
import json
import logging as log
import shutil
import subprocess
import urllib.request
import zipfile

log.basicConfig(level=log.DEBUG)

INDEX_TEMPLATE = Template("""
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>$project_title</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {
                color: #241f31;
                font-family: Cantarell, Inter, sans-serif;
                font-size: 1.1em;
                max-width: 30em;
                margin: auto;
                padding: 1em 2em;
            }
            a {
                color: #1a5fb4;
                text-decoration: none;
            }
            a:hover, a:focus {
                text-decoration: underline;
                color: #3584e4;
            }
            footer {
                margin-top: 4em;
                font-size: 0.8em;
            }

            @media(prefers-color-scheme: dark) {
                body {
                    background-color: #241f31;
                    color: #deddda;
                }
                a {
                    color: #99c1f1;
                }
                a:hover, a:focus {
                    color: #62a0ea;
                }
            }
        </style>
    </head>
    <body>
        <h1>
            $project_title
        </h1>
        $early_section
        <h2>Documentation</h2>
        <section>
            <ul>
                $latest_stable_li
                <li><a href="git/docs">Development</a></li>
            </ul>
            <h1>Specific versions</h1>
            $version_ul
        </section>
        <h2>Miscellaneous</h2>
        <section>
            <ul>
                <li><a href="$project_url">Repository</a></li>
            </ul>
        </section>
        <footer>
            Generated $datetime by <a href="https://gitlab.gnome.org/World/Rust/gir-rustdoc">gir-rustdoc</a>
        </footer>
    </body>
</html>
""")

SECTION_TEMPLATE = Template("""
        <h2>$title</h2>
        <section>
            $content
        </section>
""")

LATEST_STABLE_LI = """<li><a href="stable/latest/docs">Latest stable</a></li>"""

VERSION_LIST_ITEM_TEMPLATE = Template("""
    <li><a href="stable/$version_name/docs">Version $version_name</a></li>
""")

VERSION_CHECKER_TEMPLATE = Template("""
<script type="text/javascript">
    document.addEventListener("DOMContentLoaded", checkDocsLatestStable);

    function checkDocsLatestStable() {
        function popup(msg, url) { return `
            <style scoped="scoped">
                .popup {
                    max-width: 16em;
                    background-color: #fff;
                    position: fixed;
                    z-index: 1;
                    margin-left: auto;
                    bottom: 1em;
                    right: 1em;
                    border-radius: 5px;
                    box-shadow: 1px 1px 4px #777;
                }

                .popup a.version {
                    color: #e57300;
                    padding: 0.6em 1em;
                    display: block;
                }
                .popup a.version:hover {
                    color: #b25900;
                }

                .popup a.close {
                    color: white;
                    background: grey;
                    border-radius: 99px;
                    display: inline-block;
                    width: 19px;
                    line-height: 19px;
                    font-weight: bold;
                    text-align: center;
                    margin: 0.6em;
                    float: right;
                }
                .popup a.close:hover {
                    background: black;
                }
            </style>
            <section class="popup" id="gir_docs_popup">
                <a class="close" href="#" onclick="document.getElementById('gir_docs_popup').remove(); return false">×</a>
                <a class="version" href="$pages_url/stable/latest/docs">
                    ⚠ $${msg}
                </a>
            </section>
        ` };

        if ("$branch" == "$default_branch") {
            document.body.insertAdjacentHTML(
                'beforeend',
                popup("This is the development version. Go to latest stable version.", "stable/latest")
            );
        } else {
            fetch('$pages_url/LATEST_RELEASE_BRANCH')
                .then(response => response.text())
                .then(latest_branch => {
                    if (latest_branch.trim() != "$branch") {
                        document.body.insertAdjacentHTML(
                            'beforeend',
                            popup("This version is outdated. Go to latest version.", "stable/latest")
                        );
                    }
                });
        }
    }
</script>
""")

BEFORE_CONTENT_FILENAME = f"{getcwd()}/_rustdoc-html-before-content.html"

def pre_docs(args):
    x = dict()
    releases = parse_releases(args)

    # write html-content file
    content = ""
    with open(BEFORE_CONTENT_FILENAME, "w") as f:
        if releases:
            content = VERSION_CHECKER_TEMPLATE.substitute(
                branch=args.branch,
                default_branch=args.default_branch,
                pages_url=args.pages_url.rstrip('/'),
            )
        print(content, file=f)

    # generate flags
    flags = getenv("RUSTDOCFLAGS", "")

    if "unstable-options" not in flags:
        flags += "\n-Z unstable-options "

    flags += f"""
      --enable-index-page
      --html-before-content {BEFORE_CONTENT_FILENAME}
    """

    # try to determine dependency versions

    metadata = subprocess.run(["cargo", "metadata", "--format-version=1"], capture_output=True, text=True)

    package_version = dict()
    try:
        metadata_json = json.loads(metadata.stdout)
        for package in metadata_json['packages']:
            name = package['name']
            version = package['version']
            if name in ['gtk', 'gtk4', 'glib']:
                if args.branch == args.default_branch:
                    package_version[name] = "git"
                else:
                    package_version[name] = "stable/" + '.'.join(version.split('.')[0:2])

    except (KeyError, json.decoder.JSONDecodeError):
        log.warn("Failed to auto detect package versions")
        pass

    def get_version(package, selected):
                if selected == "none":
                    return "none"
                elif selected == "auto":
                    return package_version.get(package, "none")
                else:
                    return selected

    version = get_version('glib', args.gtk_rs_core_version)
    if version != "none":
        core = f"https://gtk-rs.org/gtk-rs-core/{version}/docs/"

        flags += f"""
            --extern-html-root-url=cairo={core}
            --extern-html-root-url=gdk_pixbuf={core}
            --extern-html-root-url=gio={core}
            --extern-html-root-url=glib={core}
            --extern-html-root-url=graphene={core}
            --extern-html-root-url=pango={core}
        """

    version = get_version('gtk', args.gtk3_rs_version)
    if version != 'none':
        gtk3 = f"https://gtk-rs.org/gtk3-rs/{version}/docs/"

        flags += f"""
            --extern-html-root-url=gdk={gtk3}
            --extern-html-root-url=atk={gtk3}
            --extern-html-root-url=gtk={gtk3}
        """

    version = get_version('gtk4', args.gtk4_rs_version)
    if version != 'none':
        gtk4 = f"https://gtk-rs.org/gtk4-rs/{version}/docs/"

        flags += f"""
            --extern-html-root-url=gdk4={gtk4}
            --extern-html-root-url=gsk4={gtk4}
            --extern-html-root-url=gtk4={gtk4}
        """
    if getenv("GITHUB_REF"):
        print(flags)
    else:
        print(f"export RUSTDOCFLAGS={quote(flags)}")


def html_index(args):
    makedirs("public", exist_ok=True)

    releases = parse_releases(args)

    if releases:
        with open("public/LATEST_RELEASE_BRANCH", "w") as f:
            print(releases[0].branch, file=f)

        version_ul = "<ul>"
        for release in releases:
            version_ul += VERSION_LIST_ITEM_TEMPLATE.substitute(
                version_name=release.name
            )
        version_ul += "</ul>"

        latest_stable_li = LATEST_STABLE_LI
    else:
        version_ul = "<i>There are no releases yet.</i>"
        latest_stable_li = ""

    early_section = ""
    for title, content in args.early_section:
        early_section += SECTION_TEMPLATE.substitute(
            title=title,
            content=content,
        )

    with open("public/index.html", "w") as f:
        content = INDEX_TEMPLATE.substitute(
            project_url=args.project_url.rstrip('/'),
            project_title=args.project_title,
            version_ul=version_ul,
            latest_stable_li=latest_stable_li,
            early_section=early_section,
            datetime=datetime.datetime.now(datetime.timezone.utc).strftime('%c UTC')
        )
        print(content, file=f)

def docs_from_artifacts(args):
    makedirs("public/stable", exist_ok=True)

    releases = parse_releases(args)

    for n, release in enumerate(releases):

        opener = urllib.request.build_opener()
        if args.job_token:
            opener.addheaders = [('JOB-TOKEN', args.job_token)]

        url = f"{args.project_url}/-/jobs/artifacts/{release.branch}/download?job=docs"
        filename = f"artifacts-{release.branch}.zip"

        log.info(f"Downloading {url}")
        with open(filename, "bw") as f:
            with opener.open(url) as data:
                f.write(data.read())

        with zipfile.ZipFile(filename, "r") as zip:
            zip.extractall(f"public/stable/{release.name}")

            # most recent release
            if n == 0:
                symlink(f"{release.name}", "public/stable/latest")

    if args.compress:
        chdir('public')
        for filename in chain(
            iglob("**/*.html", recursive=True),
            iglob("**/*.js", recursive=True),
            iglob("**/*.css", recursive=True),
            iglob("**/*.txt", recursive=True),
        ):
            with open(filename, 'rb') as f_in:
                with gzip.open(f"{filename}.gz", 'wb') as f_out:
                    shutil.copyfileobj(f_in, f_out)



def parse_releases(args):
    releases = []
    for declaration in args.releases.split():
        (branch, name) = declaration.split("=")
        releases.append(Release(branch, name))

    return releases


class Release:
    def __init__(self, branch, name):
        self.branch = branch
        self.name = name

def github_project_url():
    base = getenv("GITHUB_SERVER_URL")
    path = getenv("GITHUB_REPOSITORY")

    if base and path:
        return f"{base}/{path}".rstrip('/')
    else:
        return None

def github_branch():
    branch = getenv("GITHUB_REF")

    is_release = getenv("GITHUB_EVENT_NAME", "push") == "release"

    if branch:
        try:
            branch = branch.split('/', 3)[2]
        except IndexError:
            pass
    
        if is_release:
            return ".".join(branch.split('.')[0:2])
        else:
            return branch
    else:
        return None

def github_title():
    name = getenv("GITHUB_REPOSITORY")
    if name:
        try:
            return name.split('/', 2)[1]
        except IndexError:
            return name
    else:
        return None

def parse_args():

    parser = argparse.ArgumentParser(description='Helps to generate docs pages for Rust gir based projects.')

    parser.add_argument("--pages-url", default=getenv("CI_PAGES_URL"), help="Base URL.")
    parser.add_argument("--branch", default=getenv("CI_COMMIT_BRANCH", github_branch()), help="Current branch.")
    parser.add_argument("--default-branch", default=getenv("CI_DEFAULT_BRANCH", "main"), help="Default branch.")
    parser.add_argument("--project-url", default=getenv("CI_PROJECT_URL", github_project_url()), help="Displayed on the index page and used for GitLab CI artifacts.")
    parser.add_argument("--project-title", default=getenv("CI_PROJECT_TITLE", github_title()), help="Displayed on the index page.")
    parser.add_argument("--releases", default=getenv("RELEASES", ""),
                            help="List of releases in the format '<branch2>=<name2> <branch1>=<name1>'. The latest release has to be first.")
    parser.add_argument("--job-token", default=getenv("CI_JOB_TOKEN"), help="GitLab CI only.")

    subparsers = parser.add_subparsers(dest='<command>', required=True)

    info = "Injects JavaScript code via RUSTDOCFLAGS for outdated version warnings."
    parser_pre_docs = subparsers.add_parser('pre-docs', description=info, help=info)
    parser_pre_docs.set_defaults(func=pre_docs)

    help = "Version used in this project. Format 'stable/<version>', 'git', 'auto' or 'none'."
    parser_pre_docs.add_argument("--gtk3-rs-version", default='auto', help=help)
    parser_pre_docs.add_argument("--gtk4-rs-version", default='auto', help=help)
    parser_pre_docs.add_argument("--gtk-rs-core-version", default='auto', help=help)

    info = "Creates public/index.html with overview of all versions. Also creates LATEST_RELEASE_BRANCH required by JavaScript version checker."
    parser_html_index = subparsers.add_parser('html-index', description=info, help=info)
    parser_html_index.add_argument("--early-section", action='append', nargs=2, default=[],
                                    help="Adds additional section early in the document. First argument is the title, second argument is arbitrary HTML code.")
    parser_html_index.set_defaults(func=html_index)

    info = "GitLab CI only. Pull artifact for each version and embed to public/. Does compression as well."
    parser_docs_from_artifacts = subparsers.add_parser('docs-from-artifacts', description=info, help=info)
    parser_docs_from_artifacts.add_argument("--compress", type=bool, default=True)
    parser_docs_from_artifacts.set_defaults(func=docs_from_artifacts)

    args = parser.parse_args()

    if not args.project_url:
        parser.error(f"No project URL given")

    if args.func == pre_docs and not args.branch:
        parser.error(f"Command 'pre-docs' requires a branch to be specified. '{args.branch}' was given.")

    return args


def main():
    args = parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
