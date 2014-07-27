/* A Bison parser, made by GNU Bison 2.3.  */

/* Skeleton interface for Bison's Yacc-like parsers in C

   Copyright (C) 1984, 1989, 1990, 2000, 2001, 2002, 2003, 2004, 2005, 2006
   Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor,
   Boston, MA 02110-1301, USA.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* Tokens.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
   /* Put the tokens into the symbol table, so that GDB and other debuggers
      know about them.  */
   enum yytokentype {
     WORD = 258,
     QWORD = 259,
     LOCAL = 260,
     LET = 261,
     FOR = 262,
     CLOSURE = 263,
     FN = 264,
     ANDAND = 265,
     BACKBACK = 266,
     EXTRACT = 267,
     CALL = 268,
     COUNT = 269,
     DUP = 270,
     FLAT = 271,
     OROR = 272,
     PRIM = 273,
     REDIR = 274,
     SUB = 275,
     NL = 276,
     ENDFILE = 277,
     ERROR = 278,
     PIPE = 279
   };
#endif
/* Tokens.  */
#define WORD 258
#define QWORD 259
#define LOCAL 260
#define LET 261
#define FOR 262
#define CLOSURE 263
#define FN 264
#define ANDAND 265
#define BACKBACK 266
#define EXTRACT 267
#define CALL 268
#define COUNT 269
#define DUP 270
#define FLAT 271
#define OROR 272
#define PRIM 273
#define REDIR 274
#define SUB 275
#define NL 276
#define ENDFILE 277
#define ERROR 278
#define PIPE 279




#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
typedef union YYSTYPE
#line 23 "./parse.y"
{
	Tree *tree;
	char *str;
	NodeKind kind;
}
/* Line 1529 of yacc.c.  */
#line 103 "y.tab.h"
	YYSTYPE;
# define yystype YYSTYPE /* obsolescent; will be withdrawn */
# define YYSTYPE_IS_DECLARED 1
# define YYSTYPE_IS_TRIVIAL 1
#endif

extern YYSTYPE yylval;

