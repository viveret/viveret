---
---
<!DOCTYPE html>
<html lang="{{ site.lang }}">
<head>
    <meta charset='utf-8'>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width,maximum-scale=2">
    <link rel="stylesheet" type="text/css" media="screen" href="/assets/css/style.css">
    
    <meta property="og:title" content="{{ if title }}{{ title }} - {{ site.title }}{{ else }}{{ site.title }}{{ endif }}" />
    <meta property="og:type" content="{{ page.type }}" />
    <meta property="og:url" content="{{ page.url }}" />
    <meta property="og:image" content="{{ if page.image }}{{ page.image }}{{ else }}{{ site.image }}{{ endif }}" />

    <title>{{ if title }}{{ title }} - {{ site.title }}{{ else }}{{ site.title }}{{ endif }}</title>

    <link rel="icon" type="image/png" href="/assets/favicon.png">

    <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/css/bootstrap.min.css" integrity="sha384-Gn5384xqQ1aoWXA+058RXPxPg6fy4IWvTNh0E263XmFcJlSAwiGgFAW/dAiS6JXm" crossorigin="anonymous">

    <link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.3.1/css/all.css" integrity="sha384-mzrmE5qonljUremFsqc01SB46JvROS7bZs3IO2EmfFsd15uHvIt+Y8vEf7N7fWAU" crossorigin="anonymous">
    <link href='https://fonts.googleapis.com/css?family=Cookie' rel='stylesheet' type='text/css'>
    <link href='https://fonts.googleapis.com/css?family=Great+Vibes' rel='stylesheet' type='text/css'>
</head>
<body>
    <div id="content-bg"></div>
    <div id="content">
      <header class="ContentContainer">
        <div id="forkme_banner">
          <a href="{{ site.github.repository_url }}">View on GitHub</a>
          <p class="copyright">Made by <a href="{{ site.github.owner_url }}">{{ site.github.owner_name }}</a></p>
          <p>Revision {{ build_revision }}</p>
          <p>(last modified: {{ datetime }})</p>
        </div>

        <div class="myPicture">
          <a class="crop" href="{{ site.url }}"><img src="/assets/images/photos/face.png"></a>
          <!--
            Could do the catchphrases again with JS / json and have the blurb animated            
            <div>"I'm Viveret and @ViewData["CurrentPhrase"]"</div>
          -->
        </div>
        <a href="{{ site.url }}" class="Title TitleLeft"><span>v</span><span>i</span><span>v</span><span>e</span><span>r</span><span>e</span><span>t</span></a>
        <a href="{{ site.url }}" class="Title TitleRight"><span>s</span><span>t</span><span>e</span><span>e</span><span>l</span><span>e</span></a>
        <div class="Subtitle"><a href="software.html" title="My Software Engineering Career">software engineer</a>, <a href="freelance.html" title="Tutoring, Mentoring, Consulting, and Freelancing">freelancer</a>, <a href="artist.html" title="I'm also an artist">artist</a> in the <a href="https://en.wikipedia.org/wiki/Seattle" target="_blank">🍎 Seattle Area 🌧️</a></div>
      </header>
      <div class="ContentContainer">
        <ul class="menu-bar">
          <li><a title="My Resume" href="/README.html"><i class="far fa-2x fa-file-alt"></i></a></li>
          <li><a title="E-Mail Me!" href="mailto:viveret.amant.official@gmail.com"><i class="far fa-2x fa-envelope"></i></a></li>
          <li><a title="My LinkedIn" href="https://www.linkedin.com/in/viveret/"><i class="fab fa-2x fa-linkedin-in"></i></a></li>
          <li><a title="My GitHub" href="https://github.com/viveret"><i class="fab fa-2x fa-github"></i></a></li>
          <li><a title="LGBTQ Lingo" href="https://viveret.github.io/lgbt-lingo/" target="_blank"><i class="fas fa-2x fa-book"></i></a></li>
        </ul>
        <ul class="menu-bar menu-bar-emoji mt-1">
          <li><a href="README.html">My Resume</a></li>
          <li><a href="projects.html">My Projects</a></li>
<!--          <li><a href="blog.html">Blog</a></li>
          <li><a href="tags.html">Pages by Tag</a></li>
          <li><a href="categories.html">Pages by Category</a></li>
-->
          <li><a href="education.html">My Education</a></li>
          <li><a href="proficiencies.html">My Skills</a></li>
        </ul>
        <ul class="menu-bar menu-bar-emoji mt-1">
          <li><a href="mckinstry.html">My Work at McKinstry</a></li>
          <li><a href="stackoverflow.html">My Work at StackOverflow</a></li>
        </ul>
      </div>

      {{ content }}

      <footer class="ContentContainer">
        <ul class="menu-bar menu-bar-emoji">
          <li><a href="https://www.linkedin.com/in/viveret/">LinkedIn</a></li>
          <li><a href="mailto:viveret.amant.official@gmail.com">viveret.amant.official@gmail.com</a></li>
          <li><a href="credits.html">Credits</a></li>
        </ul>
        <center><small><a target="_blank" href="https://github.com/viveret/viveret/commit/{{ build_revision }}">#{{ build_revision }}</a>, generated {{ datetime-pretty }}</small></center>
      </footer>
      <div class="BottomFiller"></div>
    </div>

    {{ if site.debug }}
      <script
        src="https://code.jquery.com/jquery-3.6.0.js"
        integrity="sha256-H+K7U5CnXl1h5ywQfKtSj8PCmoN9aaq30gDh27Xc0jk="
        crossorigin="anonymous"></script>
      <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js"></script>
    {{ else }}
      <script
        src="https://code.jquery.com/jquery-3.6.0.min.js"
        integrity="sha256-/xUj+3OJU5yExlq6GSYGSHk7tPXikynS7ogEvDej/m4="
        crossorigin="anonymous"></script>
      <script 
        src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/js/bootstrap.min.js"
        integrity="sha384-JZR6Spejh4U02d8jOt6vLEHfe/JQGiRRSQQxSfFWpi1MquVdAyjUar5+76PVCmYl"
        crossorigin="anonymous"></script>
    {{ endif }}
</body>
</html>
